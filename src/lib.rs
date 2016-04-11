#![no_std]
#![feature(lang_items)]

#![allow(non_camel_case_types)]

#[macro_use]
extern crate syscall;

pub mod time;
pub mod unistd;

mod c_types;
pub mod syscall_mgt;

pub use platform::*;

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
