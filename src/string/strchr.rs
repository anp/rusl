use c_types::{c_schar, c_uchar, c_int};
use string::strchrnul::__strchrnul;

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const c_schar, c: c_int) -> *const c_schar {
    let r = __strchrnul(s, c);
    if *(r as *const c_uchar) == c as c_uchar {
        r
    } else {
        0 as *const c_schar
    }
}
