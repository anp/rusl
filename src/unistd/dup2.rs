use c_types::*;
use platform::errno::EBUSY;
use syscall_mgt::syscall_return;

// TODO impl for non-dup2 systems (e.g. aarch64, or1k)
#[no_mangle]
pub extern "C" fn dup2(old: c_int, new: c_int) -> c_int {
    let mut r;
    loop {
        r = unsafe { syscall!(DUP2, old, new) };
        if (r as c_int) != -EBUSY {
            break;
        }
    }
    unsafe { syscall_return(r) as c_int }
}
