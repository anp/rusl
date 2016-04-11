use c_types::*;

#[no_mangle]
pub unsafe extern "C" fn chown(path: *const c_schar, uid: uid_t, gid: gid_t) -> c_int {
    syscall!(CHOWN, path, uid, gid) as c_int
}
