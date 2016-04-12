use c_types::*;

use string::strcpy::strcpy;

const DEV_TTY: &'static [u8] = b"/dev/tty\0";

#[no_mangle]
pub unsafe extern "C" fn ctermid(s: *mut c_schar) -> *mut c_schar {
    match s as usize {
        0 => DEV_TTY.as_ptr() as *mut i8,
        _ => strcpy(s, DEV_TTY.as_ptr() as *const i8),
    }
}
