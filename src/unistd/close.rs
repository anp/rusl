use c_types::*;
use errno::EINTR;
use syscall_mgt::syscall_return;

#[no_mangle]
pub unsafe extern "C" fn close(fd: c_int) -> c_int {
    let mut r = syscall!(CLOSE, fd) as c_int;
    if r == -EINTR {
        r = 0;
    }
    syscall_return(r as usize) as c_int
}
