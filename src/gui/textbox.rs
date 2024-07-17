use std::{clone, collections::binary_heap::Iter};

use crate::uielement::{UIBuffer, UIScene, View};
use ttf_parser::*;
use vek::*;

enum TextAlignment {
    Leading,
    Center,
    Trailing,
}
pub struct TextBox {
    pub position: Vec2<u32>,
    pub width: usize,
    pub height: usize,
    pub text: UIString,
}
impl TextBox {
    pub fn new(position: Vec2<u32>, width: usize, height: usize, text: UIString) -> Self {
        return TextBox {
            position,
            width,
            height,
            text,
        };
    }
    pub fn set_text(&mut self, text: UIString) {
        self.text = text;
    }
}
impl View for TextBox {
    fn draw(&self, buffer: &mut UIBuffer) {
        let mut draw_off_x = 0f32;
        let ppi = 224f32;

        for char in &self.text.chars {
            let face = Face::parse(&char.font, 0).unwrap();
            let font_size = 10f32;
            let line_spacing = 50;
            let line_height = face.ascender() - face.descender() + line_spacing;
            let px_size =
                (line_height as f32 * font_size * ppi / (72f32 * face.units_per_em() as f32));
            let glyph_id = face.glyph_index(char.char).unwrap();
            let mut builder = Builder::new();
            let bbox: Option<ttf_parser::Rect> = face.outline_glyph(glyph_id, &mut builder);

            if bbox == None {
                draw_off_x += face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
                continue;
            }
            let off = Vec2::new(
                self.position.x as f32 + draw_off_x,
                self.position.y as f32 + (face.ascender() as f32 / px_size),
            );
            let mut pen: Vec2<u32> = Vec2::new(0, 0);
            for i in 0..builder.points.len() {
                let point = &builder.points[i];
                let next: Vec2<u32> = Vec2::new(
                    point.position[0] / px_size + off.x,
                    -point.position[1] / px_size + off.y,
                )
                .as_();
                match point.point_type {
                    PointType::Move => {
                        pen = next;
                    }
                    PointType::Line => {
                        buffer.draw_line(pen, next, char.stroke);
                        pen = next;
                    }
                    PointType::Quad => {
                        let vec1: Vec2<u32> = Vec2::new(
                            point.position[0] / px_size + off.x,
                            -point.position[1] / px_size + off.y,
                        )
                        .as_();
                        let vec: Vec2<u32> = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        )
                        .as_();

                        buffer.draw_line(pen, vec1, char.stroke);
                        pen = vec1;
                        buffer.draw_line(pen, vec, char.stroke);
                        pen = vec;
                    }
                    PointType::Curve => {
                        let vec2: Vec2<u32> = Vec2::new(
                            point.position[0] / px_size + off.x,
                            -point.position[1] / px_size + off.y,
                        )
                        .as_();
                        let vec1: Vec2<u32> = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        )
                        .as_();
                        let vec: Vec2<u32> = Vec2::new(
                            point.position[4] / px_size + off.x,
                            -point.position[5] / px_size + off.y,
                        )
                        .as_();

                        buffer.draw_line(pen, vec, char.stroke);
                        pen = vec;
                        buffer.draw_line(pen, vec1, char.stroke);
                        pen = vec1;
                        buffer.draw_line(pen, vec2, char.stroke);
                        pen = vec2
                    }
                }
            }
            draw_off_x += face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
            // buffer.draw_rect(
            //     self.position,
            //     self.position + Vec2::new(self.width as u32, self.height as u32),
            //     char.stroke,
            // );
            // buffer.draw_line(
            //     self.position + draw_off_x,
            //     (self.position
            //         + draw_off_x * Vec2::unit_x()
            //         + line_height as f32 / px_size * -Vec2::unit_y()) as u32,
            //     char.stroke,
            // );
        }
    }
    fn abs_pos(&self) -> vek::Vec2<u32> {
        self.position
    }
    fn pos(&self) -> vek::Vec2<u32> {
        self.position
    }
    fn children(&self) -> Vec<&dyn View> {
        vec![]
    }
}

pub trait RectCorners {
    fn tr(&self) -> Vec2<i16>;
    fn tl(&self) -> Vec2<i16>;
    fn br(&self) -> Vec2<i16>;
    fn bl(&self) -> Vec2<i16>;
}
impl RectCorners for ttf_parser::Rect {
    fn tr(&self) -> Vec2<i16> {
        return Vec2::new(self.x_max, self.y_max);
    }
    fn tl(&self) -> Vec2<i16> {
        return Vec2::new(self.x_min, self.y_max);
    }
    fn br(&self) -> Vec2<i16> {
        return Vec2::new(self.x_max, self.y_min);
    }
    fn bl(&self) -> Vec2<i16> {
        return Vec2::new(self.x_min, self.y_min);
    }
}
#[derive(Debug)]
pub enum PointType {
    Move,
    Line,
    Quad,
    Curve,
}
#[derive(Debug)]
pub struct OutlinePoint {
    pub position: Vec<f32>,
    pub point_type: PointType,
}
impl OutlinePoint {
    pub fn new(position: Vec<f32>, point_type: PointType) -> Self {
        OutlinePoint {
            position,
            point_type,
        }
    }
}
pub struct Builder {
    pub points: Vec<OutlinePoint>,
}
impl Builder {
    pub fn new() -> Self {
        return Builder { points: vec![] };
    }
}
impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x, y], PointType::Move))
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x, y], PointType::Line))
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x1, y1, x, y], PointType::Quad))
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.points.push(OutlinePoint::new(
            vec![x1, y1, x2, y2, x, y],
            PointType::Curve,
        ))
    }
    fn close(&mut self) {
        //write!(&mut self.0, "Z ").unwrap();
    }
}
pub trait Position {
    fn pos_from_xy(&mut self, x: u32, y: u32, width: usize) -> &mut u32;
}
impl Position for Vec<u32> {
    fn pos_from_xy(&mut self, x: u32, y: u32, width: usize) -> &mut u32 {
        let len = self.len() - 1;
        &mut self[(y as usize * width + x as usize).clamp(0, len)]
    }
}
#[derive(Clone)]
pub struct Character {
    pub char: char,
    pub ital: bool,
    pub bold: bool,
    pub point_size: f32,
    pub highlight: Rgb<u8>,
    pub stroke: Rgb<u8>,
    pub font: Vec<u8>,
}

impl Character {
    pub fn new(
        char: char,
        ital: bool,
        bold: bool,
        point_size: f32,
        highlight: Rgb<u8>,
        stroke: Rgb<u8>,
        font: Vec<u8>,
    ) -> Self {
        Self {
            char,
            ital,
            bold,
            point_size,
            highlight,
            stroke,
            font,
        }
    }
}
pub struct UIString {
    chars: Vec<Character>,
}
impl UIString {
    pub fn new(chars: Vec<Character>) -> Self {
        Self { chars }
    }
    pub fn from_str(str: &str, font: Vec<u8>, color: Rgb<u8>, size: f32) -> Self {
        let mut chars: Vec<Character> = vec![];
        for char in str.chars() {
            let c = Character::new(char, false, false, size, Rgb::black(), color, font.clone());
            chars.push(c);
        }
        Self::new(chars)
    }
}
impl IntoIterator for UIString {
    type Item = Character;
    type IntoIter = std::vec::IntoIter<Character>;
    fn into_iter(self) -> Self::IntoIter {
        self.chars.into_iter()
    }
}
