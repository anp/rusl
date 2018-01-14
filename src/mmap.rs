use core::isize;
use core::ptr;

use va_list::VaList;

use c_types::*;
use errno::{set_errno, EPERM, EINVAL, ENOMEM};
use platform::mman::*;
use thread::vmlock::__vm_wait;

use syscall_mgt::syscall_return;

#[no_mangle]
pub unsafe extern "C" fn __munmap(start: *mut c_void, len: size_t) -> c_int {
    __vm_wait();
    syscall!(MUNMAP, start, len) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn __mmap(start: *mut c_void,
                                len: size_t,
                                prot: c_int,
                                flags: c_int,
                                fd: c_int,
                                off: off_t)
                                -> *mut c_void {

    if (off & (PAGE_SIZE - 1)) != 0 {
        set_errno(EINVAL);
        return MAP_FAILED;
    }

    if len >= isize::MAX as usize {
        set_errno(ENOMEM);
        return MAP_FAILED;
    }

    if flags & MAP_FIXED != 0 {
        __vm_wait();
    }

    // Ported this fixup from
    // http://git.musl-libc.org/cgit/musl/commit/src/mman/mmap.c?id=da438ee1fc516c41ba1790cef7be551a9e244397
    let mut ret = syscall!(MMAP, start, len, prot, flags, fd, off);
    if ret == -EPERM as usize && start == ptr::null_mut() && (flags & MAP_ANON) != 0 && (flags & MAP_FIXED) == 0 {
        ret = -ENOMEM as usize;
    }
    syscall_return(ret) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn __mremap(old_address: *mut c_void,
                                  old_len: size_t,
                                  new_len: size_t,
                                  flags: c_int,
                                  mut args: VaList)
                                  -> *mut c_void {

    let new_address = if flags & MREMAP_FIXED != 0 {
        __vm_wait();
        Some(args.get::<usize>())
    } else {
        None
    };

    mremap_helper(old_address, old_len, new_len, flags, new_address)
}

pub unsafe fn mremap_helper(old_address: *mut c_void,
                            old_len: size_t,
                            new_len: size_t,
                            flags: c_int,
                            new_address: Option<usize>)
                            -> *mut c_void {

    if new_len >= isize::MAX as usize {
        set_errno(ENOMEM);
        return MAP_FAILED;
    }

    let new_address = new_address.unwrap_or(0);

    syscall!(MREMAP, old_address, old_len, new_len, flags, new_address) as *mut c_void
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn madvise(address: *mut c_void, len: usize, advice: c_int) -> c_int {
    syscall!(MADVISE, address, len, advice) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn __madvise(address: *mut c_void, len: usize, advice: c_int) -> c_int {
    madvise(address, len, advice)
}

#[no_mangle]
pub unsafe extern "C" fn mincore(address: *mut c_void, len: usize, vec: *mut u8) -> c_int {
    syscall!(MINCORE, address, len, vec) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn mlock(address: *const c_void, len: usize) -> c_int {
    syscall!(MLOCK, address, len) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn mlockall(flags: c_int) -> c_int { syscall!(MLOCKALL, flags) as c_int }


// aliases
#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn munmap(start: *mut c_void, len: size_t) -> c_int { __munmap(start, len) }

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn mmap(start: *mut c_void,
                              len: size_t,
                              prot: c_int,
                              flags: c_int,
                              fd: c_int,
                              off: off_t)
                              -> *mut c_void {
    __mmap(start, len, prot, flags, fd, off)
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn mmap64(start: *mut c_void,
                                len: size_t,
                                prot: c_int,
                                flags: c_int,
                                fd: c_int,
                                off: off_t)
                                -> *mut c_void {
    __mmap(start, len, prot, flags, fd, off)
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn mremap(old_address: *mut c_void,
                                old_len: size_t,
                                new_len: size_t,
                                flags: c_int,
                                args: VaList)
                                -> *mut c_void {
    __mremap(old_address, old_len, new_len, flags, args)
}
