lazy_static! {
    // this init is behind a spinlock, should be fine
    pub static ref AUXV_PTR: usize = unsafe { auxv_ptr() as usize };
}

unsafe fn environ() -> *mut *const *const i8 {
    extern "C" {
        static mut environ: *const *const i8;
    }
    &mut environ
}

unsafe fn auxv_ptr() -> *mut *const *const i8 {
    // using sample at bottom of
    // http://articles.manugarg.com/aboutelfauxiliaryvectors
    // basically, the auxv pointer starts at the end of the envp block
    // so we get envp, go until it's NULL, and then go one past
    let mut ptr = environ();

    while *ptr as usize != 0 {
        ptr = ptr.offset(1);
    }

    // *ptr is now NULL, go one past
    ptr = ptr.offset(1);

    ptr
}
