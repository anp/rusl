pub unsafe extern fn __brk(new_break: usize) -> usize {
    syscall!(BRK, new_break)
}
