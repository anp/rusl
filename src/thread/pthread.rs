use c_types::*;
use signal::NSIG;

pub use platform::pthread::*;

const SIGMASK_LEN: usize = (NSIG as usize / 8) / LONG_BYTES;

#[repr(C)]
pub struct pthread {
    this: *mut pthread,

    dtv: *mut *mut c_void,
    unused1: *mut c_void,
    unused2: *mut c_void,

    sysinfo: usize,
    canary: usize,
    canary2: usize,
    tid: pid_t,
    pid: pid_t,

    tsd_used: c_int,
    pub errno_val: c_int,

    // TODO(adam) make these atomic
    cancel: c_int,
    canceldisable: c_int,
    cancelasync: c_int,

    detached: c_int,
    map_base: *mut c_uchar,
    map_size: usize,
    stack: *const c_void,
    stack_size: usize,

    start_arg: *mut c_void,
    // void *(*start)(void *);
    start: *mut c_void,
    result: *mut c_void,

    cancelbuf: *mut __ptcb,
    tsd: *mut c_void,

    // TODO(adam) waiting on proper union support to implement correctly
    // pthread_attr_t attr;
    attr: [c_int; 14],
    dead: c_int,
    robust_list: rb_list,

    unblock_cancel: c_int,
    timer_id: c_int,

    // TODO(adam) track down declaration of locale_t when relevant
    locale: *mut c_void,
    killlock: [c_int; 2],
    exitlock: [c_int; 2],
    startlock: [c_int; 2],

    sigmask: [usize; SIGMASK_LEN],
    dlerror_buf: *mut c_schar,
    dlerror_flag: c_int,
    stdio_locks: *mut c_void,
    canary_at_end: usize,
    dtv_copy: *mut *mut c_void,
}

#[repr(C)]
pub struct rb_list {
    head: *mut c_void,
    off: c_longlong,
    pending: *mut c_void,
}

#[repr(C)]
pub struct __ptcb {
    // void (*__f)(void *);
    __f: *mut c_void,
    __x: *mut c_void,
    __next: *mut __ptcb,
}
