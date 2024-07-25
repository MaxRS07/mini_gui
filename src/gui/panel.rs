use crate::{
    uielement::{UIBuffer, View},
    utils::enccol,
};
use vek::*;
type Color = Rgb<u8>;
pub struct Panel<'a> {
    pub bounds: Aabr<u32>,
    pub style: PanelStyle,
    pub children: Vec<&'a dyn View>,
}
impl<'a> Panel<'a> {
    pub fn new(bounds: Aabr<u32>, style: PanelStyle) -> Self {
        Panel {
            bounds,
            style,
            children: vec![],
        }
    }
}
impl<'a> View for Panel<'a> {
    fn abs_pos(&self) -> Vec2<u32> {
        self.bounds.min
    }
    fn pos(&self) -> Vec2<u32> {
        self.bounds.min
    }
    fn children(&self) -> Vec<&dyn View> {
        self.children.clone()
    }
    fn draw(&self, buffer: &mut UIBuffer) {
        let bw = self.bounds.size().w;
        let bh = self.bounds.size().h;
        for x in 0..bw {
            for y in 0..bh {
                let _x = (self.bounds.min.x + x) as usize;
                let _y = (self.bounds.min.y + y) as usize;

                let w = self.style.stroke_width;

                if x <= w || x >= bw - w || y <= w || y >= bh - w {
                    let c = buffer.get(_x, _y);
                    *c = enccol(self.style.stroke_color);
                } else {
                    let c = buffer.get(_x, _y);
                    *c = enccol(self.style.fill_color);
                }
            }
        }
    }
    fn bbox(&self) -> Aabr<u32> {
        self.bounds
    }
}
pub struct PanelStyle {
    pub stroke_width: u32,
    pub stroke_color: Color,
    pub fill_color: Color,
}
impl PanelStyle {
    pub fn new(fill_color: Color, stroke_color: Color, stroke_width: u32) -> Self {
        return PanelStyle {
            stroke_width,
            stroke_color,
            fill_color,
        };
    }
}
