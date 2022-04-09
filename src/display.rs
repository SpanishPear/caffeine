use spin::Mutex;
use lazy_static::lazy_static;
extern crate core;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const DEPTH: i32 = 16;
const BYTES_PER_PIXEL: i32 = 4;

lazy_static! {
    pub static ref FRAME_BUFFER: Mutex<DisplayWriter> = Mutex::new(DisplayWriter::new(
        unsafe { core::slice::from_raw_parts_mut(0xA0000 as *mut u8, (WIDTH * HEIGHT * 4) as usize) }
    ));
}

pub struct DisplayWriter {
    framebuffer: &'static mut [u8],
}

impl DisplayWriter {
    pub fn new(framebuffer: &'static mut[u8]) -> DisplayWriter {
        DisplayWriter {
            framebuffer,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.framebuffer.iter_mut() {
            *pixel = 0;
        }
    }
}
