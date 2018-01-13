use core::isize;
use core::mem::transmute;
use core::ptr;

use spin::Mutex;

use c_types::*;
use errno::{set_errno, ENOMEM};
use malloc::expand_heap::__expand_heap;
use mmap::{__madvise, __mmap, __munmap, mremap_helper};
use platform::atomic::{a_and_64, a_crash, a_ctz_64, a_or_64, a_store, a_swap};
use platform::malloc::*;
use platform::mman::*;
use thread::{__wait, __wake};

pub const MMAP_THRESHOLD: usize = 0x1c00 * SIZE_ALIGN;
pub const DONTCARE: usize = 16;
pub const RECLAIM: usize = 163_840;

static mut HEAP: Heap = Heap::new();

pub struct Chunk {
    psize: usize,
    csize: usize,
    next: *mut Chunk,
    prev: *mut Chunk,
}

impl Chunk {
    pub unsafe fn trim(&mut self, n: usize) {
        let n1 = self.size();

        if n >= n1 - DONTCARE {
            return;
        }

        let next = self.next();
        let split = (self as *const Self as *mut u8).offset(n as isize) as *mut Chunk;

        (*split).psize = n | 1;
        (*split).csize = (n1 - n) | 1;

        (*next).psize = (n1 - n) | 1;

        self.csize = n | 1;

        free((*split).as_mem());
    }

    pub fn is_mmapped(&self) -> bool { (self.csize & 1) == 0 }

    pub unsafe fn from_mem(ptr: *mut c_void) -> *mut Chunk {
        ((ptr as *mut u8).offset(-(OVERHEAD as isize))) as *mut Chunk
    }

    pub unsafe fn as_mem(&self) -> *mut c_void {
        (self as *const Chunk as *const u8).offset(OVERHEAD as isize) as *mut c_void
    }

    pub fn size(&self) -> usize { self.csize & ((-2i64) as usize) }

    pub fn psize(&self) -> usize { self.psize & ((-2i64) as usize) }

    unsafe fn previous(&self) -> *mut Chunk {
        (self as *const Self as *const u8).offset(-(self.psize() as isize)) as *mut Chunk
    }

    unsafe fn next(&self) -> *mut Chunk {
        (self as *const Self as *const u8).offset(self.size() as isize) as *mut Chunk
    }
}

pub struct Bin {
    lock: [c_int; 2],
    head: *mut Chunk,
    tail: *mut Chunk,
}

pub struct Heap {
    binmap: u64,
    bins: [Bin; 64],
}

impl Bin {
    const fn new() -> Self {
        Bin {
            lock: [0; 2],
            head: ptr::null::<Chunk>() as *mut Chunk,
            tail: ptr::null::<Chunk>() as *mut Chunk,
        }
    }
}

macro_rules! array {
    (@accum (0, $($_es:expr),*) -> ($($body:tt)*))
        => {array!(@as_expr [$($body)*])};
    (@accum (2, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (0, $($es),*) -> ($($body)* $($es,)* $($es,)*))};
    (@accum (4, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (2, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (8, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (4, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (16, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (8, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (32, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (16, $($es,)* $($es),*) -> ($($body)*))};
    (@accum (64, $($es:expr),*) -> ($($body:tt)*))
        => {array!(@accum (32, $($es,)* $($es),*) -> ($($body)*))};

    (@as_expr $e:expr) => {$e};

    [$e:expr; $n:tt] => { array!(@accum ($n, $e) -> ()) };
}


impl Heap {
    const fn new() -> Self {
        Heap {
            binmap: 0,
            bins: array![Bin::new(); 64],
        }
    }

    pub unsafe fn allocate(&self, mut n: usize) -> *mut c_void {
        let mut c: *mut Chunk;

        if self.adjust_size(&mut n) < 0 {
            return ptr::null_mut();
        }

        if n > MMAP_THRESHOLD {
            let len = n + OVERHEAD + PAGE_SIZE as usize - 1 & (-PAGE_SIZE) as usize;
            let base = __mmap(ptr::null_mut(),
                              len,
                              PROT_READ | PROT_WRITE,
                              MAP_PRIVATE | MAP_ANONYMOUS,
                              -1,
                              0) as *mut u8;

            if base == ((-1isize) as usize) as *mut u8 {
                return ptr::null_mut();
            }

            c = base.offset((SIZE_ALIGN - OVERHEAD) as isize) as *mut Chunk;
            (*c).csize = len - (SIZE_ALIGN - OVERHEAD);
            (*c).psize = SIZE_ALIGN - OVERHEAD;
            return (*c).as_mem();
        }

        let i = self.bin_index_up(n);
        loop {
            let mask = self.binmap & (-((1usize << i) as isize)) as u64;
            if mask == 0 {
                c = self.expand(n);
                if c.is_null() {
                    return ptr::null_mut();
                }
                if self.alloc_rev(c) {
                    let x = c;
                    c = (*c).previous();

                    let new = (*x).csize + (*c).size();
                    (*c).csize = new;
                    (*(*x).next()).psize = new;
                }
                break;
            }

            let j = a_ctz_64(mask) as c_int;
            lock_bin(j);
            c = self.bins[j as usize].head;

            if c != self.bin_to_chunk(j as usize) {
                if !self.pretrim(c, n, i, j) {
                    self.unbin(c, j);
                }
                unlock_bin(j);
                break;
            }
            unlock_bin(j);
        }

        // Now patch up in case we over-allocated
        (*c).trim(n);

        (*c).as_mem()
    }

    pub unsafe fn reallocate(&self, p: *mut c_void, mut n: usize) -> *mut c_void {
        if p.is_null() {
            return malloc(n);
        }

        if self.adjust_size(&mut n) < 0 {
            return ptr::null_mut();
        }

        let s = Chunk::from_mem(p);
        let n0 = (*s).size();

        if (*s).is_mmapped() {
            let extra = (*s).psize;
            let base = (s as *mut u8).offset(-(extra as isize));
            let old_len = n0 + extra;
            let new_len = n + extra;

            // Crash on realloc of freed chunk
            if extra & 1 != 0 {
                a_crash();
            }

            let new = malloc(n);
            if new_len < PAGE_SIZE as usize && !new.is_null() {
                memcpy(new, p, n - OVERHEAD);
                free(p);
                return new;
            }

            let new_len = (new_len + PAGE_SIZE as usize - 1) & (-PAGE_SIZE) as usize;

            if old_len == new_len {
                return p;
            }

            let base = mremap_helper(base as *mut c_void, old_len, new_len, MREMAP_MAYMOVE, None);

            if base as usize == (-1isize) as usize {
                return if new_len < old_len {
                    p
                } else {
                    ptr::null_mut()
                };
            }

            let s = base.offset(extra as isize) as *mut Chunk;
            (*s).csize = new_len - extra;

            return (*s).as_mem();
        }

        let mut next = (*s).next();

        // Crash on corrupted footer (likely from buffer overflow)
        if (*next).psize != (*s).csize {
            a_crash();
        }

        // Merge adjacent chunks if we need more space. This is not
        // a waste of time even if we fail to get enough space, because our
        // subsequent call to free would otherwise have to do the merge.
        let mut n1 = n0;
        if n > n1 && self.alloc_fwd(next) {
            n1 += (*next).size();
            next = (*next).next();
        }

        (*s).csize = n1 | 1;
        (*next).psize = n1 | 1;

        // If we got enough space, split off the excess and return
        if n <= n1 {
            (*s).trim(n);
            return (*s).as_mem();
        }

        // As a last resort, allocate a new chunk and copy to it.
        let new = malloc(n - OVERHEAD);
        if new.is_null() {
            return ptr::null_mut();
        }

        memcpy(new, p, n0 - OVERHEAD);
        free((*s).as_mem());

        new
    }

    pub unsafe fn free_ptr(&self, ptr: *mut c_void) {
        static FREE_LOCK: Mutex<()> = Mutex::new(());

        if ptr.is_null() {
            return;
        }

        let mut s = Chunk::from_mem(ptr);
        let mut next: *mut Chunk;
        let mut final_size: usize;
        let new_size: usize;
        let mut size: usize;

        let mut reclaim = false;
        let mut i: c_int;

        if (*s).is_mmapped() {
            let extra = (*s).psize as isize;
            let base = (s as *mut u8).offset(-extra);
            let len = (*s).size() + extra as usize;

            // crash on double free
            if extra & 1 != 0 {
                a_crash();
            }
            __munmap(base as *mut c_void, len);
            return;
        }

        new_size = (*s).size();
        final_size = new_size;
        next = (*s).next();

        // crash on corrupted footer (likely from buffer overflow)
        if (*next).psize != (*s).csize {
            a_crash();
        }

        {
            let mut _held_lock = None;
            loop {
                if (*s).psize & (*next).csize & 1 != 0 {
                    (*s).csize = final_size | 1;
                    (*next).psize = final_size | 1;

                    i = self.bin_index(final_size);
                    lock_bin(i);
                    _held_lock = Some(FREE_LOCK.lock());

                    if (*s).psize & (*next).csize & 1 != 0 {
                        break;
                    }

                    _held_lock = None;
                    unlock_bin(i);
                }

                if self.alloc_rev(s) {
                    s = (*s).previous();
                    size = (*s).size();
                    final_size += size;

                    if new_size + size > RECLAIM && (new_size + size ^ size) > size {
                        reclaim = true;
                    }
                }

                if self.alloc_fwd(next) {
                    size = (*next).size();
                    final_size += size;
                    if new_size + size > RECLAIM && (new_size + size ^ size) > size {
                        reclaim = true;
                    }
                    next = (*next).next();
                }
            }

            if (self.binmap & 1u64 << i) == 0 {
                a_or_64(&self.binmap as *const _ as *mut _, 1u64 << i);
            }

            (*s).csize = final_size;
            (*next).psize = final_size;
        }

        (*s).next = self.bin_to_chunk(i as usize);
        (*s).prev = self.bins[i as usize].tail;
        (*(*s).next).prev = s;
        (*(*s).prev).next = s;

        // replace middle of large chunks with fresh zero pages
        if reclaim {
            let a = s as usize + SIZE_ALIGN + PAGE_SIZE as usize - 1 & (-PAGE_SIZE) as usize;
            let b = next as usize - SIZE_ALIGN & (-PAGE_SIZE) as usize;
            __madvise(a as *mut c_void, b - a, MADV_DONTNEED);
        }

        unlock_bin(i);
    }

    unsafe fn expand(&self, mut n: usize) -> *mut Chunk {
        static mut HEAP_LOCK: Mutex<*mut c_void> = Mutex::new(ptr::null_mut());

        let mut p: *mut c_void;
        let mut w: *mut Chunk;

        // The argument n already accounts for the caller's chunk
        // overhead needs, but if the heap can't be extended in-place,
        // we need room for an extra zero-sized sentinel chunk.
        n += SIZE_ALIGN;

        let mut end = HEAP_LOCK.lock();

        p = __expand_heap(&mut n as *mut usize);

        if p.is_null() {
            return ptr::null_mut();
        }

        // If not just expanding existing space, we need to make a
        // new sentinel chunk below the allocated space.
        if p != *end {
            // Valid/safe because of the prologue increment.
            n -= SIZE_ALIGN;
            p = (p as *mut u8).offset(SIZE_ALIGN as isize) as *mut c_void;
            w = Chunk::from_mem(p);
            (*w).psize = 1;
        }

        // Record new heap end and fill in footer.
        *end = (p as *mut u8).offset(n as isize) as *mut c_void;
        w = Chunk::from_mem(*end);
        (*w).psize = n | 1;
        (*w).csize = 1;

        // Fill in header, which may be new or may be replacing a
        // zero-size sentinel header at the old end-of-heap.
        w = Chunk::from_mem(p);
        (*w).csize = n | 1;

        w
    }

    unsafe fn alloc_fwd(&self, c: *mut Chunk) -> bool {
        let mut i: c_int;
        let mut k: usize;

        while {
            k = (*c).csize;
            k & 1 == 0
        } {
            i = self.bin_index(k);
            lock_bin(i);
            if (*c).csize == k {
                self.unbin(c, i);
                unlock_bin(i);
                return true;
            }
            unlock_bin(i);
        }

        false
    }

    unsafe fn alloc_rev(&self, c: *mut Chunk) -> bool {
        let mut i: c_int;
        let mut k: usize;

        while {
            k = (*c).psize;
            k & 1 == 0
        } {
            i = self.bin_index(k);
            lock_bin(i);
            if (*c).psize == k {
                self.unbin((*c).previous(), i);
                unlock_bin(i);
                return true;
            }
            unlock_bin(i);
        }
        false
    }

    unsafe fn unbin(&self, c: *mut Chunk, i: c_int) {
        if (*c).prev == (*c).next {
            a_and_64(&self.binmap as *const _ as *mut _, !(1u64 << i));
        }

        (*(*c).prev).next = (*c).next;
        (*(*c).next).prev = (*c).prev;
        (*c).csize |= 1;
        (*(*c).next()).psize |= 1;
    }

    unsafe fn bin_to_chunk(&self, i: usize) -> *mut Chunk {
        Chunk::from_mem(((&self.bins[i].head) as *const *mut Chunk as usize) as *mut c_void)
    }

    fn bin_index(&self, x: usize) -> i32 {
        let x = (x / SIZE_ALIGN) - 1;

        if x <= 32 {
            x as c_int
        } else if x > 0x1c00 {
            63
        } else {
            unsafe { ((transmute::<c_float, u32>((x as c_int) as c_float) >> 21) - 496) as c_int }
        }
    }

    fn bin_index_up(&self, x: usize) -> c_int {
        let x = (x / SIZE_ALIGN) - 1;

        if x <= 32 {
            x as c_int
        } else {
            unsafe {
                ((transmute::<c_float, u32>((x as c_int) as c_float) + 0x1fffff >> 21) -
                 496) as c_int
            }
        }
    }

    // pretrim - trims a chunk _prior_ to removing it from its bin.
    // Must be called with i as the ideal bin for size n, j the bin
    // for the _free_ chunk self, and bin j locked.
    unsafe fn pretrim(&self, s: *mut Chunk, n: usize, i: c_int, j: c_int) -> bool {
        // We cannot pretrim if it would require re-binning.
        if j < 40 || (j < i + 3 && j != 63) {
            return false;
        }

        let n1 = (*s).size();

        if n1 - n <= MMAP_THRESHOLD {
            return false;
        }

        if self.bin_index(n1 - n) != j {
            return false;
        }

        let next = (*s).next();
        let split = (s as *mut u8).offset(n as isize) as *mut Chunk;

        (*split).prev = (*s).prev;
        (*split).next = (*s).next;
        (*(*split).prev).next = split;
        (*(*split).next).prev = split;
        (*split).psize = n | 1;
        (*split).csize = n1 - n;

        (*next).psize = n1 - n;

        (*s).csize = n | 1;

        true
    }

    unsafe fn adjust_size(&self, n: *mut usize) -> c_int {
        // Result of pointer difference must fit in ptrdiff_t.
        if *n - 1 > isize::MAX as usize - SIZE_ALIGN as usize - PAGE_SIZE as usize {
            if *n != 0 {
                set_errno(ENOMEM);
                -1
            } else {
                *n = SIZE_ALIGN;
                0
            }
        } else {
            *n = (*n + OVERHEAD + SIZE_ALIGN - 1) & SIZE_MASK;
            0
        }
    }
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut u8;
}

#[no_mangle]
pub unsafe extern "C" fn malloc(n: usize) -> *mut c_void { HEAP.allocate(n) }

#[no_mangle]
pub unsafe extern "C" fn __malloc0(n: usize) -> *mut c_void {
    let p = malloc(n);

    let chunk = Chunk::from_mem(p);

    if !p.is_null() && !(*chunk).is_mmapped() {
        for i in 0..n {
            *(p as *mut u8).offset(i as isize) = 0;
        }
    }

    p
}

#[no_mangle]
pub unsafe extern "C" fn free(p: *mut c_void) { HEAP.free_ptr(p); }

#[no_mangle]
pub unsafe extern "C" fn realloc(p: *mut c_void, n: usize) -> *mut c_void { HEAP.reallocate(p, n) }

unsafe fn lock(lock: *mut c_int) {
    while a_swap(lock, 1) != 0 {
        __wait(lock, lock.offset(1), 1, 1);
    }
}

unsafe fn unlock(lock: *mut c_int) {
    if *lock != 0 {
        a_store(lock, 0);
        if *lock.offset(1) != 0 {
            __wake(lock as *mut c_void, 1, 1);
        }
    }
}

unsafe fn unlock_bin(i: c_int) { unlock(&mut HEAP.bins[i as usize].lock[0]); }

unsafe fn lock_bin(i: c_int) {
    let i = i as usize;
    lock(&mut HEAP.bins[i].lock[0]);

    if HEAP.bins[i].head.is_null() {
        HEAP.bins[i].tail = HEAP.bin_to_chunk(i);
        HEAP.bins[i].head = HEAP.bins[i].tail;
    }
}
