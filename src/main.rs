#![no_std]
#![no_main]
use core::panic::PanicInfo;

use crate::vga_buffer::ColorCode;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().set_color(ColorCode::new(
        vga_buffer::ForegroundColor::Green,
        vga_buffer::BackgroundColor::Black,
    ));
    crate::println!(
        "Hello World\nI am using println! to display args such as: {}",
        "!"
    );
    loop {}
}

/// This function is called on kernel panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
