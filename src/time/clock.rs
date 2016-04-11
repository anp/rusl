use core::u64;

use c_types::*;
use platform::{EINVAL, ENOSYS};
use syscall_mgt::syscall_return;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;
pub const CLOCK_SGI_CYCLE: clockid_t = 10;
pub const CLOCK_TAI: clockid_t = 11;

#[repr(C)]
pub struct tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    __tm_gmtoff: c_longlong,
    __tm_zone: *const c_schar,
}

#[repr(C)]
pub struct timespec {
    tv_sec: time_t,
    tv_nsec: c_longlong,
}

#[repr(C)]
pub struct itimerspec {
    interval: timespec,
    value: timespec,
}

#[repr(C)]
pub struct sigevent;

#[no_mangle]
pub unsafe extern "C" fn __clock_gettime(clock: clockid_t, spec: &mut timespec) -> c_int {
    clock_gettime(clock, spec)
}

#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clock: clockid_t, spec: &mut timespec) -> c_int {
    let mut r = syscall!(CLOCK_GETTIME, clock, spec as *mut timespec) as isize;

    if r == -ENOSYS {
        if clock == CLOCK_REALTIME {
            syscall!(GETTIMEOFDAY, spec as *mut timespec, 0);
            spec.tv_nsec = spec.tv_nsec * 1000;
            return 0;
        }
        r = -EINVAL;
    }

    syscall_return(r as u64) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn clock() -> clock_t {
    let mut spec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    if clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut spec) != 0 {
        return u64::MAX;

    }

    if spec.tv_sec as u64 > u64::MAX / 1_000_000 ||
       (spec.tv_nsec / 1000) as u64 > (u64::MAX - 1_000_000) * spec.tv_sec as u64 {
        return u64::MAX;
    }

    (spec.tv_sec * 1000000 + spec.tv_nsec / 1000) as clock_t
}

#[no_mangle]
pub unsafe extern "C" fn clock_getcpuclockid(pid: pid_t, clock: *mut clockid_t) -> c_int {
    let mut spec = timespec {
        tv_sec: 0, tv_nsec: 0
    };

    let id = ((-pid - 1) * 8) + 2;
    let r = syscall!(CLOCK_GETRES, id, &mut spec as *mut timespec);
    if r != 0 {
        -(r as c_int)
    } else {
        *clock = id;
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn clock_getres(clock: clockid_t, spec: &mut timespec) -> c_int {
    syscall!(CLOCK_GETRES, clock, spec as *mut timespec) as i32
}
