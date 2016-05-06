use c_types::*;
use environ::AUXV_PTR;


/// Comment from original musl C function:
///
/// This function returns true if the interval [old,new]
/// intersects the 'len'-sized interval below &libc.auxv
/// (interpreted as the main-thread stack) or below &b
/// (the current stack). It is used to defend against
/// buggy brk implementations that can cross the stack.
#[no_mangle]
pub extern "C" fn traverses_stack_p(old: usize, new: usize) -> c_int {

    let len = 8usize << 20;

    let b = *AUXV_PTR;
    let a = if b > len {
        b - len
    } else {
        0
    };

    if new > a && old < b {
        return 1;
    }

    let b = (&b as *const usize) as usize;
    let a = if b > len {
        b - len
    } else {
        0
    };

    if new > a && old < b {
        return 1;
    }

    0
}
