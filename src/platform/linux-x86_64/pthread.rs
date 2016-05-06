use thread::pthread::pthread;

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn __pthread_self() -> *mut pthread {
    let slf: *mut pthread;
    asm!("mov %fs:0, $0" : "=r" (slf) ::: "volatile");
    slf
}
