use core::usize;

use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const c_schar) -> size_t {
    // TODO(adam) convert to checking word-size chunks
    for i in 0.. {
        if *s.offset(i) == 0 {
            return i as usize;
        }
    }
    usize::MAX
}
