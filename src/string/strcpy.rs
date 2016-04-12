use c_types::*;
use string::stpcpy::stpcpy;

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut c_char, source: *const c_char) -> *mut c_char {
	stpcpy(dest, source);
	dest
}
