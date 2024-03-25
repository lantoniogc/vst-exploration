use x11rb::protocol::xproto::ImageOrder;

pub(crate) fn get_pixel8_rgba(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    bits_per_pixel: u32,
    bit_order: ImageOrder,
) -> (u8, u8, u8, u8) {
    let index = ((y * width + x) * bits_per_pixel / 8) as usize;

    let pixel = if bit_order == ImageOrder::LSB_FIRST {
        bytes[index]
    } else {
        bytes[index] & 7 << 4 | bytes[index] >> 4
    };

    let r = (pixel >> 6) as f32 / 3.0 * 255.0;
    let g = ((pixel >> 2) & 7) as f32 / 7.0 * 255.0;
    let b = (pixel & 3) as f32 / 3.0 * 255.0;

    (r as u8, g as u8, b as u8, 255)
}

pub(crate) fn get_pixel16_rgba(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    bits_per_pixel: u32,
    bit_order: ImageOrder,
) -> (u8, u8, u8, u8) {
    let index = ((y * width + x) * bits_per_pixel / 8) as usize;

    let pixel = if bit_order == ImageOrder::LSB_FIRST {
        bytes[index] as u16 | (bytes[index + 1] as u16) << 8
    } else {
        (bytes[index] as u16) << 8 | bytes[index + 1] as u16
    };

    let r = (pixel >> 11) as f32 / 31.0 * 255.0;
    let g = ((pixel >> 5) & 63) as f32 / 63.0 * 255.0;
    let b = (pixel & 31) as f32 / 31.0 * 255.0;

    (r as u8, g as u8, b as u8, 255)
}

pub(crate) fn get_pixel24_32_rgba(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    bits_per_pixel: u32,
    bit_order: ImageOrder,
) -> (u8, u8, u8, u8) {
    let index = ((y * width + x) * bits_per_pixel / 8) as usize;

    if bit_order == ImageOrder::LSB_FIRST {
        (bytes[index + 2], bytes[index + 1], bytes[index], 255)
    } else {
        (bytes[index], bytes[index + 1], bytes[index + 2], 255)
    }
}
