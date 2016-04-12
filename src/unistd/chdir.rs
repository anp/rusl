use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn chdir(path: *const c_char) -> c_int {
    syscall!(CHDIR, path) as c_int
}
