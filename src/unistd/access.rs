use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn access(filename: *const c_char, amode: c_int) -> c_int {
    syscall!(ACCESS, filename, amode) as i32
}
