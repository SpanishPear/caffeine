use core::slice;
//use alloc::slice;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use conquer_once::spin::OnceCell;
use core::convert::TryInto;
use spin::{Mutex, MutexGuard};

pub static FRAMEBUFFER: OnceCell<Mutex<Framebuffer>> = OnceCell::uninit();

pub struct Framebuffer {
    // the underlying buffer
    pub buffer: &'static mut [u8],
    // height in pixels
    pub height: usize,
    // width in pixels
    pub width: usize,
    // stride in bytes (!!)
    // Number of pixels between the start of a line and the start of the next.
    stride: usize,
    // bytes per pixel
    bytes_per_pixel: usize,
}

pub fn init_graphics(mut buffer: &mut FrameBuffer) {
    let FrameBufferInfo {
        horizontal_resolution: width,
        vertical_resolution: height,
        stride,
        bytes_per_pixel,
        ..
    } = buffer.info();

    // we need the length of the buffer for slice::from_raw_parts
    let buffer_ptr = buffer.buffer_mut().as_mut_ptr();
    let buffer_len = buffer.buffer_mut().len();

    FRAMEBUFFER.init_once(|| {
        Mutex::new(Framebuffer {
            buffer: unsafe { slice::from_raw_parts_mut(buffer_ptr, buffer_len) },
            height,
            width,
            stride: stride * bytes_per_pixel,
            bytes_per_pixel,
        })
    });
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Create's a new color from the given RGB values
    ///
    /// # Arguments
    ///
    /// * `red` - The red value
    /// * `green` - The green value
    /// * `blue` - The blue value
    pub fn from(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// Create's a color from a hex value
    pub fn hex(hex: u32) -> Color {
        Color {
            red: (hex >> 16) as u8,
            green: (hex >> 8) as u8,
            blue: hex as u8,
        }
    }
}

/// obtain the framebuffer
pub fn obtain_buffer() -> MutexGuard<'static, Framebuffer> {
    FRAMEBUFFER.get().unwrap().lock()
}

/// draw a pixel at x,y with the given color
/// this function will obtain the framebuffer and draw the pixel
/// # Arguments
/// * `x` - The x coordinates
/// * `y` - The y coordinates
/// * `color` - The color to draw
pub fn draw_pixel(x: usize, y: usize, color: Color) {
    let mut buf = obtain_buffer();
    let offset = y * buf.stride + (x * buf.bytes_per_pixel);
    set_pixel(buf.buffer, offset, color)
}

/// Draw a horizontal line from (x,y) to (x+len, y)
pub fn draw_line_horizontal(x: usize, y: usize, len: usize, color: Color) {
    let mut buf = obtain_buffer();
    assert!((x + len) <= buf.width);
    let mut offset = y * buf.stride + (x * buf.bytes_per_pixel);
    for _ in 0..len {
        set_pixel(buf.buffer, offset, color);
        offset += buf.bytes_per_pixel;
    }
}

/// draw a line from (x0, y0) to (x1, y1)
/// using Bresenham's line algorithm
/// https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut buf = obtain_buffer();

    // Create local variables for moving start point
    let mut x0 = x0;
    let mut y0 = y0;

    // Get absolute x/y offset
    let dx = if x0 > x1 { x0 - x1 } else { x1 - x0 };
    let dy = if y0 > y1 { y0 - y1 } else { y1 - y0 };

    // Get slopes
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    // Initialize error
    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        // Set pixel
        let y0_usize: usize = y0.try_into().expect("y cannot be negative");
        let x0_usize: usize = x0.try_into().expect("x cannot be negative");
        let offset = y0_usize * buf.stride + (x0_usize * buf.bytes_per_pixel);
        set_pixel(buf.buffer, offset, color);

        // Check end condition
        if x0 == x1 && y0 == y1 {
            break;
        };

        // Store old error
        err2 = 2 * err;

        // Adjust error and start position
        if err2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if err2 < dy {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draws a rectangle from starting coordinates x, y with width and height
///
/// # Arguments
///
/// * `x` - x coordinate of the top left corner
/// * `y` - y coordinate of the top left corner
/// * `w` - width of the rectangle
/// * `h` - height of the rectangle
/// * `color` - color of the rectangle
pub fn draw_rect(x: usize, y: usize, w: usize, h: usize, color: Color) {
    let mut buf = obtain_buffer();
    assert!((x + w) <= buf.width);
    assert!((y + h) <= buf.width);

    let mut line_offset = y * buf.stride + (x * buf.bytes_per_pixel);
    let mut offset = line_offset;
    for _ in 0..h {
        for _ in 0..w {
            set_pixel(buf.buffer, offset, color);
            offset += buf.bytes_per_pixel;
        }
        line_offset += buf.stride;
        offset = line_offset;
    }
}

/// Clears the screen to black
pub fn clear_screen() {
    let mut buf = obtain_buffer();
    let mut offset = 0;
    for _ in 0..buf.height {
        for _ in 0..buf.width {
            set_pixel(buf.buffer, offset, Color::hex(0x000000));
            offset += buf.bytes_per_pixel;
        }
    }
}

#[inline]
fn set_pixel(buf: &mut [u8], offset: usize, color: Color) {
    buf[offset] = color.blue;
    buf[offset + 1] = color.green;
    buf[offset + 2] = color.red;
}
