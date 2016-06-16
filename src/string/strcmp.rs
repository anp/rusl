use core::i32;

use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn strcmp(l: *const c_schar, r: *const c_schar) -> c_int {
    // TODO(adam) convert to checking word-size chunks
    for i in 0.. {
        let lc = *l.offset(i);
        let rc = *r.offset(i);
        if lc == 0 || lc != rc {
            return (lc - rc) as c_int;
        }
    }
    i32::MAX
}
