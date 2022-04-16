#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

//extern crate alloc;
extern crate core;

// macro use will export our macro across the crate
#[macro_use]
mod drivers;
mod video;
mod tests;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use crate::video::Color;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    video::init_graphics(boot_info.framebuffer.as_mut().unwrap());

    video::clear_screen();
    video::draw_rect(300, 300, 150, 150, Color::hex(0xFFC0CB));
    video::draw_line(10, 10, 300, 420, Color::hex(0xA020F0));
    video::draw_line_horizontal(5, 5, 300, Color::from(255, 0, 0));
    
    kprintln!("Hello, world!");
    caffeine::init();

    x86_64::instructions::interrupts::int3();
    
    kprintln!("after breakpoint, it lives!");
    #[cfg(test)]
    test_main();

    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("OH NO, PANIC AT THE DISCO");
    kprintln!("\t{}", info);
    loop {}
}

