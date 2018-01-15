use super::{has_zero, ONES};
use c_types::{c_int, c_schar, c_uchar, size_t, uintptr_t};
use core::mem;
use string::strlen::strlen;

#[no_mangle]
pub unsafe extern "C" fn __strchrnul(s: *const c_schar, c: c_int) -> *const c_schar {
    strchrnul(s, c)
}

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn strchrnul(s: *const c_schar, c: c_int) -> *const c_schar {
    let mut _s = s;
    let c = c as c_uchar;
    if c == 0 {
        return _s.add(strlen(_s));
    }

    while _s as uintptr_t % mem::size_of::<size_t>() != 0 {
        if *_s == 0 || *(_s as *const c_uchar) == c {
            return _s;
        }
        _s = _s.offset(1);
    }

    let k: *const size_t = (ONES * c as size_t) as *const size_t;
    let mut w: *const size_t = _s as *const size_t;
    while !has_zero(*w) && !has_zero(*w ^ (k as size_t)) {
        w = w.offset(1);
    }

    _s = w as *const c_schar;
    while *_s != 0 && *(_s as *const c_uchar) != c {
        _s = _s.offset(1);
    }
    _s
}
