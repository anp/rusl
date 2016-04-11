use core::intrinsics::transmute;

use platform::errno;

// from musl/src/internal/syscall_ret.c
pub unsafe fn syscall_return(code: u64) -> isize {
    let max_err: u64 = transmute(-4096i64);
    if code > max_err {
        errno = -(code as isize);
        -1
    } else {
        transmute(code)
    }
}
