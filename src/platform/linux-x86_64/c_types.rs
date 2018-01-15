// most of these were copied from the rust-lang libc crate
use core::i32;

#[repr(u8)]
pub enum c_void {
    // Two dummy variants so the #[repr] attribute can be used.
    #[doc(hidden)] __variant1,
    #[doc(hidden)] __variant2,
}

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub const INT_BYTES: usize = 4;
pub const LONG_BYTES: usize = 8;
pub const C_INT_MAX: i32 = i32::MAX;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;
pub type off_t = i64;

pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = size_t;
pub type cc_t = c_uchar;

pub type clock_t = u64;
pub type time_t = i64;
pub type clockid_t = c_int;
pub type suseconds_t = c_longlong;
