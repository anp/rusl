use c_types::*;

#[no_mangle]
pub extern "C" fn dup(fd: c_int) -> c_int {
    unsafe {
        syscall!(DUP, fd) as c_int
    }
}
