use c_types::*;
use string::stpcpy::stpcpy;

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut c_schar, source: *const c_schar) -> *mut c_schar {
	stpcpy(dest, source);
	dest
}
