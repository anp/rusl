use c_types::c_schar;
use super::strlen::strlen;
use malloc::malloc::malloc;
use core::ptr;
use memcpy;

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn strdup(s: *const c_schar) -> *const c_schar {
    let l = strlen(s);
    let d = malloc(l + 1);
    if d.is_null() {
        ptr::null()
    } else {
        memcpy(d as *mut u8, s as *const u8, l + 1) as *const c_schar
    }
}

#[no_mangle]
pub unsafe extern "C" fn __strdup(s: *const c_schar) -> *const c_schar { strdup(s) }
