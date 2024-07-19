use core::f32;
use std::cmp;

use ttf_parser::{math, Rect};
use vek::{Rgb, Vec2, Vec3};

use crate::{gui::textbox::RectCorners, utils::enccol};

#[derive(Clone)]
pub struct UIScene<'a> {
    pub children: Vec<&'a dyn View>,
}
impl<'a> UIScene<'a> {
    pub fn new() -> Self {
        UIScene { children: vec![] }
    }
    pub fn add_child(&mut self, child: &'a dyn View) {
        self.children.push(child);
    }
}
impl<'a> View for UIScene<'a> {
    fn abs_pos(&self) -> Vec2<u32> {
        Vec2::zero()
    }
    fn pos(&self) -> Vec2<u32> {
        Vec2::zero()
    }
    fn draw(&self, buffer: &mut UIBuffer) {
        let children = self.children.clone();
        for child in children {
            child.draw(buffer);
        }
    }
    fn children(&self) -> Vec<&dyn View> {
        self.children.clone()
    }
}
pub trait View {
    fn draw(&self, buffer: &mut UIBuffer);
    fn abs_pos(&self) -> Vec2<u32>;
    fn pos(&self) -> Vec2<u32>;
    fn children(&self) -> Vec<&dyn View>;
}
pub struct UIBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}
impl UIBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0u32; width * height],
        }
    }
    pub fn get(&mut self, x: usize, y: usize) -> &mut u32 {
        let idx = (y * self.width + x).clamp(0, self.width * self.height - 1);
        &mut self.buffer[idx]
    }
    pub fn get_vec(&mut self, vec: Vec2<u32>) -> &mut u32 {
        let idx =
            (vec.y as usize * self.width + vec.x as usize).clamp(0, self.width * self.height - 1);
        &mut self.buffer[idx]
    }
    pub fn draw_line(&mut self, start: Vec2<f32>, end: Vec2<f32>, color: Rgb<u8>) {
        let dist = start.distance(end);
        let dir = (end - start).normalized();
        for i in 0..=dist as u32 {
            let pos = start + dir * i as f32;
            *self.get_vec(pos.as_()) = enccol(color);
        }
    }
    pub fn draw_line_u32(&mut self, start: Vec2<u32>, end: Vec2<u32>, color: Rgb<u8>) {
        let s: Vec2<f32> = start.as_();
        let e: Vec2<f32> = end.as_();
        self.draw_line(s, e, color)
    }
    pub fn draw_box(&mut self, min: Vec2<f32>, max: Vec2<f32>, color: Rgb<u8>) {
        let (x1, y1) = (min.x as u32, min.y as u32);
        let (x2, y2) = (max.x as u32, max.y as u32);
        let min_x = cmp::min(x1, x2);
        let min_y = cmp::min(y1, y2);
        let max_y = cmp::max(y1, y2);
        let max_x = cmp::max(x1, x2);
        let h = max_y - min_y;
        let w = max_x - min_x;
        for x in 0..=w as u32 {
            self.draw_line(
                Vec2::new((min_x + x) as f32, min_y as f32).as_(),
                Vec2::new((min_x + x) as f32, (min_y + h) as f32).as_(),
                color,
            )
        }
    }
    pub fn draw_rect(&mut self, rect: Rect, color: Rgb<u8>, fill: bool) {
        if fill {
            self.draw_line_u32(rect.tr().as_(), rect.tl().as_(), color);
            self.draw_line_u32(rect.tl().as_(), rect.bl().as_(), color);
            self.draw_line_u32(rect.bl().as_(), rect.br().as_(), color);
            self.draw_line_u32(rect.br().as_(), rect.tr().as_(), color);
        } else {
            self.draw_box(rect.tl().as_(), rect.br().as_(), color);
        }
    }
}
