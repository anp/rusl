use c_types::*;

use string::strcpy::strcpy;

const DEV_TTY: &'static [u8] = b"/dev/tty\0";

#[no_mangle]
pub unsafe extern "C" fn ctermid(s: *mut c_char) -> *mut c_char {
    match s as usize {
        0 => DEV_TTY.as_ptr() as *mut c_char,
        _ => strcpy(s, DEV_TTY.as_ptr() as *const c_char),
    }
}
