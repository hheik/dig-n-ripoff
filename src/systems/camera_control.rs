use std::f32::consts::PI;

use crate::{
    resources::{Camera, Input, MouseButton, Time},
    util::{math::lerp, Vector2F, Vector2I},
};
use specs::{Read, System, Write};

pub struct CameraControl {
    drag_start: Option<Vector2I>,
}

impl CameraControl {
    pub fn new() -> CameraControl {
        CameraControl { drag_start: None }
    }
}

impl<'a> System<'a> for CameraControl {
    type SystemData = (Read<'a, Time>, Read<'a, Input>, Write<'a, Camera>);

    fn run(&mut self, (time, input, mut camera): Self::SystemData) {
        if input.mouse_pressed(MouseButton::Middle) {
            self.drag_start =
                Some(camera.transform.get_position().rounded() + input.get_mouse_position());
        }
        if input.mouse_released(MouseButton::Middle) {
            self.drag_start = None;
        }

        if input.mouse_held(MouseButton::Middle) && self.drag_start.is_some() {
            camera.transform.set_position(Vector2F::from(
                self.drag_start.unwrap() - input.get_mouse_position(),
            ));
        }

        // let lifetime = time.lifetime.elapsed().unwrap().as_secs_f32();
        // let t = (lifetime * 0.5).sin() * 0.5 + 0.5;
        // camera
        //     .transform
        //     .set_rotation(lerp(PI / 32.0, -PI / 32.0, t));

        // camera.transform.set_scale(Vector2F {
        //     // x: (lifetime.sin() * 0.25 + 0.75) * 4.0,
        //     // y: (lifetime.cos() * 0.25 + 0.75) * 4.0,
        //     x: (lifetime.sin() * 0.10 + 0.90) * 4.0,
        //     y: (lifetime.cos() * 0.10 + 0.90) * 4.0,
        //     // x: (lifetime.sin() * 0.01 + 0.99) * 4.0,
        //     // y: (lifetime.cos() * 0.01 + 0.99) * 4.0,
        // });

        // camera.transform.set_position(Vector2F {
        //     x: ((lifetime * 2.0).sin() * 0.5 + 0.5) * 128.0 - 128.0,
        //     y: ((lifetime * 2.0).cos() * 0.5 + 0.5) * 128.0 - 128.0,
        // });
    }
}
