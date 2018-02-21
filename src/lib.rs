#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(const_unique_new, const_atomic_usize_new)]
#![no_std]

#[macro_use]
extern crate alloc;

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
extern crate x86_64;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate once;

#[macro_use]
mod vga_buffer;

mod memory;
use memory::BumpAllocator;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    use memory::FrameAllocator;

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();

    memory::init(boot_info);

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("heap allocated.");

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("HAMOOOOOON:\nF:{}\nL:{}", file, line);
    println!("{}", fmt);
    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};
    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};
    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
    HEAP_START + HEAP_SIZE);