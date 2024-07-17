use std::u32;

use vek::Rgb;

pub fn enccol(color: Rgb<u8>) -> u32 {
    (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32
}
pub fn deccol(color: u32) -> Rgb<u8> {
    let b = (color & 0x000000FF) as u8;
    let g = ((color & 0x0000FF00) >> 8) as u8;
    let r = ((color & 0x00FF0000) >> 16) as u8;
    Rgb::new(r, g, b)
}
