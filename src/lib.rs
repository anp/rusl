#![no_std]
#![feature(lang_items)]

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
