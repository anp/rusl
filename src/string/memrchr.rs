use c_types::{c_int, c_void, size_t};
use core::ptr;

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn memrchr(m: *const c_void, c: c_int, n: size_t) -> *const c_void {
    let mut i = n as isize;
    let s = m as *const u8;
    let c = c as u8;
    while i > 0 {
        i -= 1;
        if *s.offset(i) == c {
            return s.offset(i) as *const c_void;
        }
    }
    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn __memrchr(m: *const c_void, c: c_int, n: size_t) -> *const c_void {
    memrchr(m, c, n)
}
