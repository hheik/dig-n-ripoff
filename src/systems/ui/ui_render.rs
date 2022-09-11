use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{ui::TextElement, RenderTarget};

pub struct UIRender;

impl UIRender {
    pub fn new() -> Self {
        UIRender {}
    }
}

impl<'a> System<'a> for UIRender {
    type SystemData = (
        WriteStorage<'a, RenderTarget<'static>>,
        ReadStorage<'a, TextElement>,
    );

    fn run(&mut self, (mut render_target, text): Self::SystemData) {
        for (render_target, element) in (&mut render_target, &text).join() {
            let font = element
                .get_font()
                .lock()
                .expect("Failed to lock font mutex");
            render_target.set_surface(
                match font.render(element.get_text()).solid(element.get_color()) {
                    Ok(surface) => surface,
                    Err(error) => panic!("Failed to render text: {error:?}"),
                },
            );
        }
    }
}
