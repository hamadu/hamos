#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() -> ! {
  use core::fmt::Write;

  vga_buffer::WRITER.lock().write_str("Hello!\nHello!!\nHello!!!");
  write!(vga_buffer::WRITER.lock(), "Home: {}-{}", 33, 4);

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
