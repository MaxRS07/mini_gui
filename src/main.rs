use mini_gui::{
    fonts::Fonts,
    gui::{
        panel::{Panel, PanelStyle},
        textbox::{TextBox, UIString},
    },
    uielement::{UIBuffer, UIScene, View},
    window::start,
};
use ttf_parser::Width;
use vek::{Aabr, Rgb, Vec2};
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 500;

    let mut scene = UIScene::new();
    let uistring = UIString::from_str("He", Fonts::roboto(), Rgb::white(), 65.0);
    let text_box = TextBox::new(Vec2::zero(), 200, 200, uistring);
    scene.add_child(&text_box);
    // let mut buffer = UIBuffer::new(WIDTH, HEIGHT);
    // text_box.draw(&mut buffer);
    start(WIDTH, HEIGHT, &mut scene)
}
