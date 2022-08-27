use std::f32::consts::PI;

use crate::{
    resources::{Camera, Time},
    util::{math::lerp, Vector2F},
};
use specs::{Read, System, Write};

pub struct CameraControl;
impl<'a> System<'a> for CameraControl {
    type SystemData = (Read<'a, Time>, Write<'a, Camera>);

    fn run(&mut self, (time, mut camera): Self::SystemData) {
        camera.transform.set_position(-Vector2F::ONE * 0.0);
        let lifetime = time.lifetime.elapsed().unwrap().as_secs_f32();

        let t = (lifetime).sin() * 0.5 + 0.5;
        camera.transform.set_rotation(lerp(PI / 8.0, -PI / 8.0, t));

        camera.transform.set_scale(Vector2F {
            x: (lifetime.sin() * 0.5 + 0.5) * 4.0,
            y: (lifetime.cos() * 0.5 + 0.5) * 4.0,
        });

        camera.transform.set_position(Vector2F {
            x: ((lifetime * 2.0).sin() * 0.5 + 0.5) * 128.0 - 128.0,
            y: ((lifetime * 2.0).cos() * 0.5 + 0.5) * 128.0 - 128.0,
        });
    }
}
