use std::process::Child;
use vek::{Rgb, Vec2};

use crate::utils::enccol;

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
        let idx = (y * self.width + x).clamp(0, self.width * self.height);
        &mut self.buffer[idx]
    }
    pub fn get_vec(&mut self, vec: Vec2<u32>) -> &mut u32 {
        let idx = (vec.y as usize * self.width + vec.x as usize).clamp(0, self.width * self.height);
        &mut self.buffer[idx]
    }
    pub fn draw_line(&mut self, start: Vec2<u32>, end: Vec2<u32>, color: Rgb<u8>) {
        let dv: Vec2<f32> = (end - start).as_();
        let dnorm: Vec2<u32> = (dv / dv.magnitude()).as_();
        for i in 0..dv.magnitude() as u32 {
            *self.get_vec(start + dnorm * i) = enccol(color);
        }
    }
    pub fn draw_rect(&mut self, min: Vec2<u32>, max: Vec2<u32>, color: Rgb<u8>) {
        let w = max.x - min.x;
        let h = max.y - min.y;
        for x in 0..w {
            for y in 0..h {
                *self.get((min.x + x) as usize, (min.y + y) as usize) = enccol(color);
            }
        }
    }
}
