use c_types::c_int;
use super::_Exit::_Exit;

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __funcs_on_quick_exit() {}

#[no_mangle]
pub extern "C" fn quick_exit(code: c_int) {
    __funcs_on_quick_exit();
    _Exit(code)
}
