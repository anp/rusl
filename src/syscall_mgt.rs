use core::mem::transmute;

use c_types::*;
use errno::set_errno;

// from musl/src/internal/syscall_ret.c
pub unsafe fn syscall_return(code: usize) -> usize {
    let max_err: usize = transmute(-4096i64);
    if code > max_err {
        set_errno(-(code as c_int));
        (-1isize) as usize
    } else {
        code
    }
}

// from the syscall.rs crate, just added the return handling
#[macro_export]
macro_rules! syscall {
    ($nr:ident)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall0(
                ::syscall::nr::$nr)) );

    ($nr:ident, $a1:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall1(
                ::syscall::nr::$nr,
                $a1 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall2(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall3(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall4(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall5(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall6(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize)) );

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr)
        => ( ::syscall_mgt::syscall_return(::syscall::syscall7(
                ::syscall::nr::$nr,
                $a1 as usize, $a2 as usize, $a3 as usize,
                $a4 as usize, $a5 as usize, $a6 as usize,
                $a7 as usize)) );
}
