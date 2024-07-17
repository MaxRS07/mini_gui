use std::{fs::File, io::Read};
use ttf_parser::Face;

fn from_path(path: &str) -> Vec<u8> {
    let mut font_file = File::open(path).unwrap();
    let mut font_data = Vec::new();
    font_file.read_to_end(&mut font_data).unwrap();
    font_data
}
pub struct Fonts;

impl Fonts {
    pub fn roboto() -> Vec<u8> {
        from_path("assets/Roboto-Regular.ttf")
    }
}
