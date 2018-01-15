use c_types::{c_void, size_t};
use memset;

#[no_mangle]
pub unsafe extern "C" fn bzero(s: *mut c_void, n: size_t) {
    memset(s as *mut u8, 0, n);
}
