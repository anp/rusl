use c_types::*;
use core::sync::atomic::{AtomicPtr, Ordering};

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_cas(p: *mut c_int, mut t: c_int, s: c_int) -> c_int {
    asm!("lock ; cmpxchgl $3, $1" :
         "=A"(t), "=*m"(p) :
         "A"(t), "r"(s) :
         "memory" :
         "volatile");
    t
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_cas_p(p: *mut c_int, mut t: *mut c_int, s: *mut c_int) -> *mut c_void {
    asm!("lock ; cmpxchg $3, $1" :
         "=A"(t), "=*m"(p) :
         "A"(t), "r"(s) :
         "memory" :
         "volatile");
    t as *mut c_void
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_swap(p: *mut c_int, mut v: c_int) -> c_int {
    *AtomicPtr::new(p).swap(&mut v, Ordering::Relaxed)
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_store(p: *mut c_int, x: c_int) {
    asm!("mov $1, $0 ; lock ; orl $$0, (%rsp)"
         : "=*m"(p) : "r"(x) : "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_inc(p: *mut c_int) {
    asm!(
        "lock ; incl $0"
        :"=*m"(p)
        :"m"(*p)
        :"memory"
        :"volatile"
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_dec(p: *mut c_int) {
    asm!(
        "lock ; decl $0"
        :"=*m"(p)
        :"m"(*p)
        :"memory"
        :"volatile"
    );
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_fetch_add(p: *mut c_int, mut v: c_int) -> c_int {
    asm!("lock ; xadd $0, $1"
		: "=r"(v), "=*m"(p) : "0"(v) : "memory" : "volatile");
    v
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_and(p: *mut c_int, v: c_int) {
    asm!("lock ; and $1, $0"
		 : "=*m"(p) : "r"(v) : "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_or(p: *mut c_int, v: c_int) {
    asm!("lock ; or $1, $0"
		 : "=*m"(p) : "r"(v) : "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_and_64(p: *mut u64, v: u64) {
    asm!("lock ; and $1, $0"
		 : "=*m"(p) : "r"(v) : "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_or_64(p: *mut u64, v: u64) {
    asm!("lock ; or $1, $0"
		 : "=*m"(p) : "r"(v) : "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_crash() {
    asm!("hlt" ::: "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_spin() {
    asm!("pause" ::: "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_barrier() {
    asm!("" ::: "memory" : "volatile");
}

#[inline(always)]
#[no_mangle]
pub unsafe extern "C" fn a_ctz_64(mut x: u64) -> u64 {
    asm!("bsf $1, $0" : "=r"(x) : "r"(x));
    x
}
