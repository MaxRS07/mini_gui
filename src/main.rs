use mini_gui::{
    fonts::Fonts,
    gui::{
        panel::{Panel, PanelStyle},
        textbox::{TextBox, UIString},
    },
    uielement::UIScene,
    window::start,
};
use ttf_parser::Face;
use vek::{vec2, Aabr, Rgb, Vec2};
fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 500;

    let mut scene = UIScene::new();
    let blue = PanelStyle::new(Rgb::cyan(), Rgb::blue(), 5);
    let bounds = Aabr {
        min: Vec2::zero(),
        max: Vec2::one() * 150,
    };
    let panel = Panel::new(bounds, blue);
    let uistring = UIString::from_str("Hello World", Fonts::roboto(), Rgb::white(), 10.0);
    let text_box = TextBox::new(Vec2::zero(), 200, 200, uistring);
    scene.add_child(&panel);
    scene.add_child(&text_box);
    start(WIDTH, HEIGHT, &mut scene)
}
