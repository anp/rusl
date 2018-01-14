use c_types::{c_schar, c_int};
use super::strrchr::strrchr;

#[no_mangle]
pub unsafe extern "C" fn rindex(s: *const c_schar, c: c_int) -> *const c_schar {
    strrchr(s, c)
}
