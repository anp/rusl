use atomic::{a_fetch_add, a_inc};
use c_types::*;
use thread::{__wait, __wake};

static mut LOCK: [c_int; 2] = [0, 0];

#[linkage = "weak"]
#[no_mangle]
pub unsafe extern "C" fn __vm_wait() {
    let mut tmp = LOCK[0];
    while tmp != 0 {
        __wait(
            &mut LOCK[0] as *mut c_int,
            &mut LOCK[1] as *mut c_int,
            tmp,
            1,
        );
        tmp = LOCK[0];
    }
}

#[no_mangle]
pub unsafe extern "C" fn __vm_lock() {
    a_inc(&mut LOCK[0] as *mut c_int);
}

#[no_mangle]
pub unsafe extern "C" fn __vm_unlock() {
    let result = a_fetch_add(&mut LOCK[0] as *mut c_int, -1);

    if result == 1 && LOCK[1] != 0 {
        __wake((&mut LOCK[0] as *mut c_int) as *mut c_void, -1, 1);
    }
}
