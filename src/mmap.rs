use c_types::*;

#[no_mangle]
pub unsafe extern fn madvise(address: *mut c_void, len: usize, advice: c_int) -> c_int {
    syscall!(MADVISE, address, len, advice) as c_int
}

#[no_mangle]
pub unsafe extern fn __madvise(address: *mut c_void, len: usize, advice: c_int) -> c_int {
    madvise(address, len, advice)
}

#[no_mangle]
pub unsafe extern fn mincore(address: *mut c_void, len: usize, vec: *mut u8) -> c_int {
    syscall!(MINCORE, address, len, vec) as c_int
}

#[no_mangle]
pub unsafe extern fn mlock(address: *const c_void, len: usize) -> c_int {
    syscall!(MLOCK, address, len) as c_int
}

#[no_mangle]
pub unsafe extern fn mlockall(flags: c_int) -> c_int {
    syscall!(MLOCKALL, flags) as c_int
}
