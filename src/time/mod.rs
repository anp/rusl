pub mod clock;

use c_types::*;

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
pub struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
pub struct itimerval {
	it_interval: timeval,
	it_value: timeval,
}

#[repr(C)]
pub struct sigevent;
