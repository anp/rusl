use c_types::c_int;

#[no_mangle]
pub extern "C" fn _Exit(ec: c_int) -> ! {
    unsafe {
        syscall!(EXIT_GROUP, ec);
        syscall!(EXIT, ec);
    }
    // The loop, while technically unreachable, ensures this function
    // is divergent. The syscall above returns (), so this code
    // wouldn't typecheck if we didn't have this infinite loop. See
    // this musl changeset for similar discussion:
    // http://git.musl-libc.org/cgit/musl/commit/src/exit/_Exit.c?id=0c05bd3a9c165cf2f0b9d6fa23a1f96532ddcdb3
    loop {}
}
