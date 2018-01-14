use memmove;
use c_types::{c_void, size_t};

#[no_mangle]
pub unsafe extern "C" fn bcopy(s1: *const c_void, s2: *mut c_void, n: size_t) {
    memmove(s2 as *mut u8, s1 as *const u8, n);
}
