use c_types::{c_int, c_void, size_t};
use memcmp;

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int {
    memcmp((s1 as *const u8), (s2 as *const u8), n) as c_int
}
