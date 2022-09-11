use std::time::SystemTime;

use specs::{Join, Read, System, WriteStorage};

use crate::{
    components::ui::TextElement,
    resources::{Time, UnsafeBox2D},
};

pub struct DebugInfo {
    last_display: Option<SystemTime>,
}

impl DebugInfo {
    pub fn new() -> Self {
        DebugInfo { last_display: None }
    }
}

impl<'a> System<'a> for DebugInfo {
    type SystemData = (
        WriteStorage<'a, TextElement>,
        Read<'a, UnsafeBox2D>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut text_element, box2d, time): Self::SystemData) {
        match self.last_display {
            Some(last_display) => {
                if SystemTime::now()
                    .duration_since(last_display)
                    .unwrap()
                    .as_secs_f32()
                    > 1.0
                {
                    for (element) in (&mut text_element).join() {
                        element.set_text(
                            format!("fps: {}", (1.0 / time.delta_time.as_secs_f32()).round())
                                .as_str(),
                        );
                    }
                    self.last_display = Some(SystemTime::now());
                }
            }
            None => self.last_display = Some(SystemTime::now()),
        }
    }
}
