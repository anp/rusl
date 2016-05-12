use core::isize;

use spin::Mutex;

use c_types::*;
use errno::{set_errno, ENOMEM};
use malloc::expand_heap::__expand_heap;
use platform::atomic::a_and_64;
use platform::malloc::*;
use platform::mman::*;

pub const MMAP_THRESHOLD: usize = 0x1c00 * SIZE_ALIGN;
pub const DONTCARE: usize = 16;
pub const RECLAIM: usize = 163_840;

pub struct chunk {
    psize: usize,
    csize: usize,
    next: *mut chunk,
    prev: *mut chunk,
}

pub struct bin {
    lock: [c_int; 2],
    head: *mut chunk,
    tail: *mut chunk,
}

pub struct mal {
    binmap: u64,
    bins: [bin; 64],
    free_lock: [c_int; 2],
}

extern "C" {
    static mut mal: mal;
    fn lock(lk: *mut c_int);
    fn unlock(lk: *mut c_int);
    fn lock_bin(i: c_int);
    fn unlock_bin(i: c_int);
    fn first_set(x: u64) -> c_int;
    fn bin_index(s: usize) -> c_int;
    fn bin_index_up(x: usize) -> c_int;

    fn malloc(n: usize) -> *mut c_void;
    fn __malloc0(n: usize) -> *mut c_void;
    fn realloc(p: *mut c_void, n: usize) -> *mut c_void;
    fn free(p: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn adjust_size(n: *mut usize) -> c_int {
    // Result of pointer difference must fit in ptrdiff_t.
    if *n - 1 > isize::MAX as usize - SIZE_ALIGN as usize - PAGE_SIZE as usize {
        if *n != 0 {
            set_errno(ENOMEM);
            return -1;
        } else {
            *n = SIZE_ALIGN;
            return 0;
        }
    }

    *n = (*n + OVERHEAD + SIZE_ALIGN - 1) & SIZE_MASK;
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn unbin(c: *mut chunk, i: c_int) {
    if (*c).prev == (*c).next {
        a_and_64(&mut mal.binmap, !(1u64 << i));
    }

    (*(*c).prev).next = (*c).next;
    (*(*c).next).prev = (*c).prev;
    (*c).csize |= 1;
    (*next_chunk(c)).psize |= 1;
}

#[no_mangle]
pub unsafe extern "C" fn alloc_fwd(c: *mut chunk) -> c_int {
    let mut i: c_int;
    let mut k: usize;

    while {
        k = (*c).csize;
        k & 1 == 0
    } {
        i = bin_index(k);
        lock_bin(i);
        if (*c).csize == k {
            unbin(c, i);
            unlock_bin(i);
            return 1;
        }
        unlock_bin(i);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn alloc_rev(c: *mut chunk) -> c_int {
    let mut i: c_int;
    let mut k: usize;

    while {
        k = (*c).psize;
        k & 1 == 0
    } {
        i = bin_index(k);
        lock_bin(i);
        if (*c).psize == k {
            unbin(previous_chunk(c), i);
            unlock_bin(i);
            return 1;
        }
        unlock_bin(i);
    }
    0
}

// pretrim - trims a chunk _prior_ to removing it from its bin.
// Must be called with i as the ideal bin for size n, j the bin
// for the _free_ chunk self, and bin j locked.
#[no_mangle]
pub unsafe extern "C" fn pretrim(s: *mut chunk, n: usize, i: c_int, j: c_int) -> c_int {
    // We cannot pretrim if it would require re-binning.
    if j < 40 || (j < i + 3 && j != 63) {
        return 0;
    }

    let n1 = chunk_size(s);

    if n1 - n <= MMAP_THRESHOLD {
        return 0;
    }

    if bin_index(n1-n) != j { return 0; }

    let next = next_chunk(s);
    let split = (s as *mut u8).offset(n as isize) as *mut chunk;

    (*split).prev = (*s).prev;
    (*split).next = (*s).next;
    (*(*split).prev).next = split;
    (*(*split).next).prev = split;
    (*split).psize = n | 1;
    (*split).csize = n1 - n;

    (*next).psize = n1 - n;

    (*s).csize = n | 1;

    return 1;
}

#[no_mangle]
pub unsafe extern "C" fn trim(s: *mut chunk, n: usize) {
    let n1 = chunk_size(s);

    if n >= n1 - DONTCARE { return; }

    let next = next_chunk(s);
    let split = (s as *mut u8).offset(n as isize) as *mut chunk;

    (*split).psize = n | 1;
    (*split).csize = (n1 - n) | 1;

    (*next).psize = (n1 - n) | 1;

    (*s).csize = n | 1;

    free(chunk_to_mem(split));
}

#[no_mangle]
pub unsafe extern "C" fn expand_heap(mut n: usize) -> *mut chunk {
    static mut heap_lock: Mutex<*mut c_void> = Mutex::new(0 as *mut c_void);

    let mut p: *mut c_void;
    let mut w: *mut chunk;

    // The argument n already accounts for the caller's chunk
    // overhead needs, but if the heap can't be extended in-place,
    // we need room for an extra zero-sized sentinel chunk.
    n += SIZE_ALIGN;

    let mut end = heap_lock.lock();

    p = __expand_heap(&mut n as *mut usize);

    if p as usize == 0 {
        return 0 as *mut chunk;
    }

    // If not just expanding existing space, we need to make a
    // new sentinel chunk below the allocated space.
    if p != *end {
        // Valid/safe because of the prologue increment.
        n -= SIZE_ALIGN;
        p = (p as *mut u8).offset(SIZE_ALIGN as isize) as *mut c_void;
        w = mem_to_chunk(p);
        (*w).psize = 0 | 1;
    }

    // Record new heap end and fill in footer.
    *end = (p as *mut u8).offset(n as isize) as *mut c_void;
    w = mem_to_chunk(*end);
    (*w).psize = n | 1;
    (*w).csize = 0 | 1;

    // Fill in header, which may be new or may be replacing a
    // zero-size sentinel header at the old end-of-heap.
    w = mem_to_chunk(p);
    (*w).csize = n | 1;

    w
}

unsafe fn mem_to_chunk(ptr: *mut c_void) -> *mut chunk {
    ((ptr as *mut u8).offset(-(OVERHEAD as isize))) as *mut chunk
}

unsafe fn chunk_to_mem(c: *mut chunk) -> *mut c_void {
    (c as *mut u8).offset(OVERHEAD as isize) as *mut c_void
}

unsafe fn chunk_size(c: *mut chunk) -> usize { (*c).csize & ((-2i64) as usize) }

unsafe fn chunk_psize(c: *mut chunk) -> usize { (*c).psize & ((-2i64) as usize) }

unsafe fn previous_chunk(c: *mut chunk) -> *mut chunk {
    (c as *mut u8).offset(-(chunk_psize(c) as isize)) as *mut chunk
}

unsafe fn next_chunk(c: *mut chunk) -> *mut chunk {
    (c as *mut u8).offset(chunk_size(c) as isize) as *mut chunk
}
