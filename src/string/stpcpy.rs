use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn stpcpy(dest: *mut c_char, source: *const c_char) -> *mut c_char {
	// TODO(adam) compare and copy as word-size chunks

	for i in 0.. {
		*dest.offset(i) = *source.offset(i);
		if *dest.offset(i) == 0 { break; }
	}

    dest
}
