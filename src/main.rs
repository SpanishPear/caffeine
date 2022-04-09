#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod display;
mod qemu;
mod serial;
mod tests;

use tests::Testable;
entry_point!(kernel_main);


fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    clear_screen();
    /*if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        serial_println!("{:?}", framebuffer.info());
        for (i, byte) in framebuffer.buffer_mut().into_iter().enumerate() {
            let byte_pos = i % 4;
            match byte_pos {
                0 => *byte = 255,
                1 => *byte = 204,
                2 => *byte = 255,
                3 => *byte = 00,
                _ => unreachable!(),
            }
        }
    }*/

    serial_println!("{:?}", boot_info);

    #[cfg(test)]
    test_main();

    loop {}
}

fn clear_screen() {
    display::FRAME_BUFFER.lock().clear();
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
