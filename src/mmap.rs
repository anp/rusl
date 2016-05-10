use core::isize;

use c_types::*;
use errno::{set_errno, EINVAL, ENOMEM};
use platform::mman::*;
use thread::vmlock::__vm_wait;

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

    syscall!(MMAP, start, len, prot, flags, fd, off) as *mut c_void
}

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
#[no_mangle]
pub unsafe extern "C" fn munmap(start: *mut c_void, len: size_t) -> c_int { __munmap(start, len) }

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
