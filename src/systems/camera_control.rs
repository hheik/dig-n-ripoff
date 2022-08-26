use crate::{resources::{Time, Camera}, util::Vector2F};
use specs::{System, Write, Read};

pub struct CameraControl;
impl<'a> System<'a> for CameraControl {
    type SystemData = (
        Read<'a, Time>,
        Write<'a, Camera>,
    );

    fn run(&mut self, (time, mut camera): Self::SystemData) {
        camera.transform.set_position(-Vector2F::ONE * 256.0);
        let lifetime = time.lifetime.elapsed().unwrap().as_secs_f32();
        camera.transform.set_scale(Vector2F::ONE * (lifetime.sin() * 0.5 + 0.5) * 4.0);
    }
}
