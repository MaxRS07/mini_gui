use crate::{
    uielement::{UIBuffer, View},
    utils::*,
    Margin,
};
use rand::prelude::*;
use std::ops::Add;
use std::os::unix;
use std::{io::Lines, ops::Div};
use ttf_parser::*;
use vek::{Rgb, Vec2};

enum TextAlignment {
    Leading,
    Center,
    Trailing,
}
pub struct TextBox<'a> {
    pub position: Vec2<u32>,
    pub width: usize,
    pub height: usize,
    pub text: UIString,
    pub parent: &'a dyn View,
}
impl<'a> TextBox<'a> {
    pub fn new(
        left: Margin,
        top: Margin,
        width: usize,
        height: usize,
        text: UIString,
        parent: &'a dyn View,
    ) -> Self {
        let _left = match left {
            Margin::Pct(pct) => (parent.bbox().size().w as f32 * pct) as u32,
            Margin::Px(px) => px,
        };
        let _top = match top {
            Margin::Pct(pct) => (parent.bbox().size().h as f32 * pct) as u32,
            Margin::Px(px) => px,
        };
        let mut position = parent.pos() + Vec2::new(_left, _top);

        let position = parent.pos()
            + return TextBox {
                position,
                width,
                height,
                text,
                parent,
            };
    }
    pub fn set_text(&mut self, text: UIString) {
        self.text = text;
    }
}
impl<'a> View for TextBox<'a> {
    fn draw(&self, buffer: &mut UIBuffer) {
        let mut draw_off_x = 0f32;
        let ppi = 224f32;
        for char in &self.text.chars {
            let mut lines: Vec<LineSegment> = vec![];
            let face = Face::parse(&char.font, 0).unwrap();
            let line_spacing = 50;
            let line_height = face.ascender() - face.descender() + line_spacing;
            let px_size = line_height as f32 * ppi / (char.point_size * face.units_per_em() as f32);
            let glyph_id = face.glyph_index(char.char).unwrap();
            let mut builder = Builder::new();
            let bbox: Option<ttf_parser::Rect> = face.outline_glyph(glyph_id, &mut builder);

            if bbox == None {
                draw_off_x += face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
                continue;
            }
            let off = Vec2::new(
                (self.position.x + self.parent.pos().x) as f32 + draw_off_x,
                (self.position.y + self.parent.pos().y) as f32 as f32
                    + (face.ascender() as f32 / px_size)
                    + face.descender() as f32 / px_size,
            );
            let mut pen: Vec2<f32> = Vec2::new(0.0, 0.0);
            for i in 0..builder.points.len() {
                let point = &builder.points[i];
                let next = Vec2::new(
                    point.position[0] / px_size + off.x,
                    -point.position[1] / px_size + off.y,
                );
                match point.point_type {
                    PointType::Move => {
                        pen = next;
                    }
                    PointType::Line => {
                        buffer.draw_line(pen, next, char.stroke);
                        lines.push(LineSegment::new(pen, next));
                        pen = next;
                    }
                    PointType::Quad => {
                        let vec = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        );

                        buffer.draw_line(pen, next, char.stroke);
                        pen = next;
                        buffer.draw_line(pen, vec, char.stroke);
                        pen = vec;
                    }
                    PointType::Curve => {
                        let vec1 = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        );
                        let vec = Vec2::new(
                            point.position[4] / px_size + off.x,
                            -point.position[5] / px_size + off.y,
                        );

                        buffer.draw_line(pen, vec, char.stroke);
                        pen = vec;
                        buffer.draw_line(pen, vec1, char.stroke);
                        pen = vec1;
                        buffer.draw_line(pen, next, char.stroke);
                        pen = next
                    }
                }
            }
            let bb = bbox.unwrap();
            let w = bb.width() as f32 / px_size;
            let h = bb.height() as f32 / px_size;
            for x in 0..=w as u32 {
                for y in 0..=h as u32 {
                    let pos: Vec2<f32> = off
                        + Vec2::new(
                            x as f32 + bb.x_min as f32 / px_size,
                            -(y as f32 + bb.y_min as f32 / px_size),
                        );
                    //*buffer.get_vec(pos.as_()) = enccol(char.stroke);
                    let a = Vec2::new(off.x, pos.y);
                    let l = LineSegment::new(pos, a);
                    let mut count = 0;
                    for line in lines.iter() {
                        if line.intersects(&l) {
                            count += 1;
                        }
                    }
                    if count % 2 == 1 {
                        *buffer.get_vec(pos.as_()) = enccol(char.stroke);
                    }
                }
            }
            draw_off_x += face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
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
    fn bbox(&self) -> vek::Aabr<u32> {
        Aabr {
            min: self.pos(),
            max: self.pos() + Vec2::new(self.width as u32, self.height as u32),
        }
    }
}
