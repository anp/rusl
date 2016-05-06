#![no_std]
#![feature(asm, lang_items, linkage)]

#![allow(non_camel_case_types)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate syscall;

pub mod malloc;
pub mod mmap;
pub mod string;
pub mod syscall_mgt;
pub mod thread;
pub mod time;
pub mod unistd;

//#[cfg(all(target_os="linux", target_arch="x86"))]
//#[path="platform/linux-x86/mod.rs"]
//pub mod platform;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

//#[cfg(all(target_os="freebsd", target_arch="x86_64"))]
//#[path="platform/freebsd-x86_64/mod.rs"]
//pub mod platform;

//#[cfg(all(target_os="linux", target_arch="arm"))]
//#[path="platform/linux-armeabi/mod.rs"]
//pub mod platform;

//#[cfg(all(target_os="macos", target_arch="x86_64"))]
//#[path="platform/macos-x86_64/mod.rs"]
//pub mod platform;

pub mod atomic {
    pub use platform::atomic::*;
}

pub mod c_types {
    pub use platform::c_types::*;
}

pub mod environ {
    pub use platform::environ::*;
}

pub mod errno {
    pub use platform::errno::*;
}

pub mod mman {
    pub use platform::mman::*;
}

pub mod pthread {
    pub use platform::pthread::*;
}

pub mod signal {
    pub use platform::signal::*;
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
