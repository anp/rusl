use core::{ usize, mem };
use super::has_zero;

use c_types::{ c_schar, size_t };

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_schar) -> size_t {
    let mut t = s;
    while t as usize % mem::size_of::<size_t>() != 0 {
        if *t == 0 {
            return t as size_t - s as size_t
        }
        t = t.offset(1)
    }

    let mut w = t as *const size_t;
    while !has_zero(*w) {
        w = w.offset(1)
    }

    t = w as *const c_schar;
    while *t != 0 {
        t = t.offset(1)
    }
    t as size_t - s as size_t
}
