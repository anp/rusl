use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn acct(filename: *const c_schar) -> c_int {
    syscall!(ACCT, filename) as i32
}
