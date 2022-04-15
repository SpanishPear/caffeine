#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

//extern crate alloc;
extern crate core;

// macro use will export our macro across the crate
#[macro_use]
mod serial;
mod video;
mod qemu;
mod tests;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use tests::Testable;
use crate::video::Color;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    video::init_graphics(boot_info.framebuffer.as_mut().unwrap());

    video::clear_screen();
    video::draw_rect(300, 300, 150, 150, Color::hex(0xFFC0CB));
    video::draw_line(10, 10, 300, 420, Color::hex(0xA020F0));
    video::draw_line_horizontal(5, 5, 300, Color::from(255, 0, 0));

    serial_println!("{:?}", boot_info);

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
}
