use c_types::*;

use time::{itimerval, timeval, ITIMER_REAL};

#[no_mangle]
pub unsafe extern "C" fn alarm(seconds: c_uint) -> c_uint {
    let mut it = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: seconds as time_t,
            tv_usec: 0,
        },
    };

    syscall!(
        SETITIMER,
        ITIMER_REAL,
        &mut it as *mut itimerval,
        &mut it as *mut itimerval
    );

    (it.it_value.tv_sec + !!it.it_value.tv_usec) as c_uint
}
