use mini_gui::{
    fonts::Fonts,
    gui::{
        panel::{Panel, PanelStyle},
        textbox::TextBox,
    },
    pct, px,
    uielement::{UIBuffer, UIScene, View},
    utils::UIString,
    window::start,
};
use ttf_parser::Width;
use vek::{Aabr, Rgb, Vec2};
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 500;

    let mut scene = UIScene::new(WIDTH as u32, HEIGHT as u32);
    let uistring = UIString::from_str("%", Fonts::roboto(), Rgb::white(), 65.0);
    let binding = scene.clone();
    let text_box = TextBox::new(pct(0.5), pct(0.5), 200, 200, uistring, &binding);
    scene.add_child(&text_box);
    // let mut buffer = UIBuffer::new(WIDTH, HEIGHT);
    // text_box.draw(&mut buffer);
    start(WIDTH, HEIGHT, &mut scene)
}
