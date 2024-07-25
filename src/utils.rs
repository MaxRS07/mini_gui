use core::ops::Add;
use std::u32;
use vek::{Rgb, Vec2};

pub fn enccol(color: Rgb<u8>) -> u32 {
    (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32
}
pub fn deccol(color: u32) -> Rgb<u8> {
    let b = (color & 0x000000FF) as u8;
    let g = ((color & 0x0000FF00) >> 8) as u8;
    let r = ((color & 0x00FF0000) >> 16) as u8;
    Rgb::new(r, g, b)
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
    pub chars: Vec<Character>,
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
#[derive(Debug)]
pub struct LineSegment {
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
}
impl LineSegment {
    pub fn new(start: Vec2<f32>, end: Vec2<f32>) -> Self {
        LineSegment { start, end }
    }
    pub fn intersects(&self, other: &LineSegment) -> bool {
        let (x1, y1) = (self.start.x, self.start.y);
        let (x2, y2) = (self.end.x, self.end.y);
        let (x3, y3) = (other.start.x, other.start.y);
        let (x4, y4) = (other.end.x, other.end.y);

        let m1 = (y2 - y1) / (x2 - x1);
        let b1 = y1 - m1 * x1;
        let m2 = (y4 - y3) / (x4 - x3);
        let b2 = y3 - m2 * x3;

        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;

        if (x1 <= x && x <= x2) || (x2 <= x && x <= x1) {
            if (y1 <= y && y <= y2) || (y2 <= y && y <= y1) {
                if (x3 <= x && x <= x4) || (x4 <= x && x <= x3) {
                    if (y3 <= y && y <= y4) || (y4 <= y && y <= y3) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
impl Add<Vec2<f32>> for LineSegment {
    type Output = LineSegment;

    fn add(self, rhs: Vec2<f32>) -> Self::Output {
        LineSegment::new(self.start + rhs, self.end + rhs)
    }
}
