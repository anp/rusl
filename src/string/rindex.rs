use super::strrchr::strrchr;
use c_types::{c_int, c_schar};

#[no_mangle]
pub unsafe extern "C" fn rindex(s: *const c_schar, c: c_int) -> *const c_schar {
    strrchr(s, c)
}
