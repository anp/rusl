#![no_std]
#![feature(asm, const_fn, lang_items, linkage)]

#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate syscall;
extern crate va_list;

#[macro_use]
pub mod syscall_mgt;

pub mod malloc;
pub mod mmap;
pub mod string;
pub mod thread;
pub mod time;
pub mod unistd;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

pub use platform::atomic as atomic;
pub use platform::c_types as c_types;
pub use platform::errno as errno;
pub use platform::environ as environ;
pub use platform::mman as mman;
pub use platform::pthread as pthread;
pub use platform::signal as signal;
