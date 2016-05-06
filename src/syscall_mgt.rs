use core::intrinsics::transmute;

use c_types::*;
use errno::set_errno;

// from musl/src/internal/syscall_ret.c
pub unsafe fn syscall_return(code: usize) -> isize {
    let max_err: usize = transmute(-4096i64);
    if code > max_err {
        set_errno(-(code as c_int));
        -1
    } else {
        transmute(code)
    }
}
