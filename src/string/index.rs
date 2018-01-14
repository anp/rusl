use c_types::{c_schar, c_int};
use string::strchr::strchr;

#[no_mangle]
pub unsafe extern "C" fn index(s: *const c_schar, c: c_int) -> *const c_schar {
    return strchr(s, c);
}
