use lazy_static::lazy_static;
use sdl2::ttf::{Font, Sdl2TtfContext};
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{text_element::TextElement, RenderTarget};

const FONT_PATH: &str = "./assets/fonts/VeniceClassic.ttf";

pub struct UIRender {
    font: Font<'static, 'static>,
}

impl UIRender {
    pub fn new() -> UIRender {
        let font_context = sdl2::ttf::init().expect("Could not initialize ttf font context");
        let font = font_context
            .load_font(FONT_PATH, 19)
            .expect("Could not load font");
        UIRender { font }
    }
}

impl<'a> System<'a> for UIRender {
    type SystemData = (
        WriteStorage<'a, RenderTarget<'static>>,
        ReadStorage<'a, TextElement>,
    );

    fn run(&mut self, (mut render_target, text): Self::SystemData) {
        for (render_target, text) in (&mut render_target, &text).join() {}
    }
}
