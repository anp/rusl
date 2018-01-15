use c_types::{c_int, c_schar};
use string::strchr::strchr;

#[no_mangle]
pub unsafe extern "C" fn index(s: *const c_schar, c: c_int) -> *const c_schar {
    strchr(s, c)
}
