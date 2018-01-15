pub mod expand_heap;
pub mod malloc;

pub unsafe extern "C" fn __brk(new_break: usize) -> usize {
    syscall!(BRK, new_break)
}
