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

    fn run(&mut self, (transform, render_target, camera, canvas): Self::SystemData) {
        println!("*** Running Render ***");
        let mut canvas = match canvas {
            Some(canvas) => canvas,
            None => return,
        };

        renderer::begin_draw(&mut canvas);
        for (transform, render_target) in (&transform, &render_target).join() {
            let pos = transform.get_position();
            let src = render_target.surface.rect();
            let dst_start = camera.transform.xform_inverse(pos).rounded();
            let (size_x, size_y) = render_target.surface.rect().size();
            let dst = Rect::new(
                dst_start.x,
                dst_start.y,
                (size_x as f32 * camera.transform.get_scale().x) as u32, // FIXME: proper transforming
                (size_y as f32 * camera.transform.get_scale().y) as u32,
            );
            renderer::draw_surface(&mut canvas, &render_target.surface, src, dst)
        }
        renderer::finish_draw(&mut canvas);
    }
}