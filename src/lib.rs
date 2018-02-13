#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;

mod vga_buffer;
use vga_buffer::print_some;

#[no_mangle]
pub extern fn rust_main() -> ! {
    let helloworld = b"Hello World!";
    let color_byte = 0x1f;
    let mut colored_world = [color_byte; 24];
    for (i, char_byte) in helloworld.into_iter().enumerate() {
        colored_world[i*2] = *char_byte;
    }

    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = colored_world };

    print_some();

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
    loop {}
}
