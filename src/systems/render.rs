use crate::{
    components::{RenderTarget, Transform},
    gl::{
        camera::Camera,
        renderer::{self, UnsafeCanvas},
    },
};
use sdl2::rect::Rect;
use specs::{Join, Read, ReadStorage, System, Write};

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, RenderTarget<'static>>,
        Read<'a, Camera>,
        Option<Write<'a, UnsafeCanvas>>,
    );

    fn run(&mut self, (transform, render_target, camera, mut canvas): Self::SystemData) {
        println!("*** Running Render ***");
        let mut canvas = match canvas {
            Some(canvas) => canvas,
            None => return,
        };

        renderer::begin_draw(&mut canvas);
        for (transform, render_target) in (&transform, &render_target).join() {
            let pos = transform.position.rounded();
            let src = render_target.surface.rect();
            let dst = Rect::new(
                pos.x * camera.scale.round() as i32,
                pos.y * camera.scale.round() as i32,
                64 * camera.scale.round() as u32,
                64 * camera.scale.round() as u32,
            );
            renderer::draw_surface(&mut canvas, &render_target.surface, src, dst)
        }
        renderer::finish_draw(&mut canvas);
    }
}
