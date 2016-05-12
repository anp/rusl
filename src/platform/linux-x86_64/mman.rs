use c_types::*;

pub const PAGE_SIZE: off_t = 4096;

pub const MAP_FAILED: *mut c_void = 0xffffffffffffffff as *mut c_void;

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 4;
pub const PROT_GROWSDOWN: c_int = 0x01000000;
pub const PROT_GROWSUP: c_int = 0x02000000;

pub const MAP_SHARED: c_int = 0x01;
pub const MAP_PRIVATE: c_int = 0x02;
pub const MAP_FIXED: c_int = 0x10;

pub const MAP_TYPE: c_int = 0x0f;
pub const MAP_FILE: c_int = 0x00;
pub const MAP_ANON: c_int = 0x20;
pub const MAP_ANONYMOUS: c_int = MAP_ANON;
pub const MAP_32BIT: c_int = 0x40;
pub const MAP_NORESERVE: c_int = 0x4000;
pub const MAP_GROWSDOWN: c_int = 0x0100;
pub const MAP_DENYWRITE: c_int = 0x0800;
pub const MAP_EXECUTABLE: c_int = 0x1000;
pub const MAP_LOCKED: c_int = 0x2000;
pub const MAP_POPULATE: c_int = 0x8000;
pub const MAP_NONBLOCK: c_int = 0x10000;
pub const MAP_STACK: c_int = 0x20000;
pub const MAP_HUGETLB: c_int = 0x40000;

pub const POSIX_MADV_NORMAL: c_int = 0;
pub const POSIX_MADV_RANDOM: c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: c_int = 2;
pub const POSIX_MADV_WILLNEED: c_int = 3;
pub const POSIX_MADV_DONTNEED: c_int = 0;

pub const MS_ASYNC: c_int = 1;
pub const MS_INVALIDATE: c_int = 2;
pub const MS_SYNC: c_int = 4;

pub const MCL_CURRENT: c_int = 1;
pub const MCL_FUTURE: c_int = 2;
pub const MCL_ONFAULT: c_int = 4;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_REMOVE: c_int = 9;
pub const MADV_DONTFORK: c_int = 10;
pub const MADV_DOFORK: c_int = 11;
pub const MADV_MERGEABLE: c_int = 12;
pub const MADV_UNMERGEABLE: c_int = 13;
pub const MADV_HUGEPAGE: c_int = 14;
pub const MADV_NOHUGEPAGE: c_int = 15;
pub const MADV_DONTDUMP: c_int = 16;
pub const MADV_DODUMP: c_int = 17;
pub const MADV_HWPOISON: c_int = 100;
pub const MADV_SOFT_OFFLINE: c_int = 101;

pub const MREMAP_MAYMOVE: c_int = 1;
pub const MREMAP_FIXED: c_int = 2;

pub const MLOCK_ONFAULT: c_int = 0x01;
