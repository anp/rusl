use c_types::*;

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn stpcpy(dest: *mut c_schar, source: *const c_schar) -> *mut c_schar {
    // TODO(adam) compare and copy as word-size chunks

    for i in 0.. {
        *dest.offset(i) = *source.offset(i);
        if *dest.offset(i) == 0 {
            break;
        }
    }

    dest
}
