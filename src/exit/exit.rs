use super::_Exit::_Exit;
use c_types::c_int;

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __funcs_on_exit() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __stdio_exit() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn _fini() {}

#[linkage = "weak"]
#[no_mangle]
pub extern "C" fn __libc_exit_fini() {
    // TODO dark magic goes here, seemingly for dynlink's benefit
    // See http://git.musl-libc.org/cgit/musl/commit/src/exit/exit.c?id=7586360badcae6e73f04eb1b8189ce630281c4b2
    // and http://git.musl-libc.org/cgit/musl/commit/src/exit/exit.c?id=19caa25d0a8e587bb89b79c3f629085548709dd4
    // for interesting discussion about how musl handles this function.
    // Really, we should iterate through __fini_array_start..__fini_array_end,
    // executing each function. But it requires annoying pointer
    // manipulation that I wasn't sure we needed yet, since we aren't
    // porting dynlink right now.
    _fini()
}

#[no_mangle]
pub extern "C" fn exit(code: c_int) -> ! {
    __funcs_on_exit();
    __libc_exit_fini();
    __stdio_exit();
    // The functions above are weakly-aliased because the dynlinker
    // might provide defs for them. Meanwhile, we provide dummy fns.
    _Exit(code)
}
