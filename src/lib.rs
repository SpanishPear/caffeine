#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod drivers;

pub mod video;
pub mod tests;

use core::panic::PanicInfo;

use tests::Testable;
use drivers::{
        interrupts::init_idt,
        qemu::{exit_qemu, QemuExitCode},
};

pub fn init() {
    init_idt();
    drivers::interrupts::gdt::init_gdt();
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    kprintln!("[failed]\n");
    kprintln!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed)
}


pub fn test_runner(tests: &[&dyn Testable]) {
    kprintln!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
