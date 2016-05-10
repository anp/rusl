use c_types::*;

pub const PAGE_SIZE: off_t = 4096;

pub const MAP_FAILED: *mut c_void = 0xffffffffffffffff as *mut c_void;

pub const MAP_FIXED: c_int = 0x10;
pub const MREMAP_FIXED: c_int = 2;
