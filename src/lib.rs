use core::f32;

use vek::num_traits::float;

pub mod fonts;
pub mod gui;
pub mod uielement;
pub mod utils;
pub mod window;

pub enum Margin {
    Px(u32),
    Pct(f32),
}
pub fn px(px: u32) -> Margin {
    Margin::Px(px)
}
pub fn pct(pct: f32) -> Margin {
    Margin::Pct(pct)
}
