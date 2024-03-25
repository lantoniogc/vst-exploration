use image::RgbaImage;
use utils::*;
use x11rb::{
    connection::Connection,
    protocol::xproto::{ConnectionExt, ImageFormat},
};

pub mod utils;

fn capture_image() -> RgbaImage {
    let (conn, _screen_num) = x11rb::connect(None).unwrap();

    let setup = conn.setup();

    let roots_len = setup.roots_len();

    let screen = &setup.roots[(roots_len - 1) as usize];

    let geometry = conn.get_geometry(screen.root).unwrap().reply().unwrap();

    let scale_factor = 1.0;

    let x = (geometry.x as f32 / scale_factor) as i32;
    let y = (geometry.y as f32 / scale_factor) as i32;
    let width = (geometry.width as f32 / scale_factor) as u32;
    let height = (geometry.height as f32 / scale_factor) as u32;

    let get_image = conn
        .get_image(
            ImageFormat::Z_PIXMAP,
            screen.root,
            x as i16,
            y as i16,
            width as u16,
            height as u16,
            u32::MAX,
        )
        .unwrap()
        .reply()
        .unwrap();

    let depth = get_image.depth;
    let data = get_image.data;

    let pixmap_format = setup
        .pixmap_formats
        .iter()
        .find(|item| item.depth == depth)
        .unwrap();

    let bits_per_pixel = pixmap_format.bits_per_pixel as u32;
    let bit_order = setup.bitmap_format_bit_order;

    let get_pixel_rgba = match depth {
        8 => get_pixel8_rgba,
        16 => get_pixel16_rgba,
        24 => get_pixel24_32_rgba,
        32 => get_pixel24_32_rgba,
        _ => panic!("Unsupported depth: {}", depth),
    };

    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for y in 0..height {
        for x in 0..width {
            let index = ((y * width + x) * 4) as usize;
            let (r, g, b, a) = get_pixel_rgba(&data, x, y, width, bits_per_pixel, bit_order);

            rgba[index] = r;
            rgba[index + 1] = g;
            rgba[index + 2] = b;
            rgba[index + 3] = a;
        }
    }

    RgbaImage::from_raw(width as u32, height as u32, rgba).unwrap()
}

fn main() {
    let image = capture_image();
    image.save("screenshot.png").unwrap();
}
