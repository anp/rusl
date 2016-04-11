use c_types::*;
use platform::EINTR;
use syscall_mgt::syscall_return;

#[no_mangle]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    let mut r = syscall!(CLOSE, fd);
    if r as isize == -EINTR { r = 0; }
    syscall_return(r) as c_int
}
