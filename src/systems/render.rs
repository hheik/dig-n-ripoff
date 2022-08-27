use crate::{
    components::{RenderTarget, Transform},
    gl::renderer::{self, UnsafeCanvas},
    resources::Camera,
    util::Vector2F,
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
        let mut canvas = match canvas {
            Some(canvas) => canvas,
            None => return,
        };

        renderer::begin_draw(&mut canvas);
        for (transform, render_target) in (&transform, &render_target).join() {
            let pos = transform.get_position();
            let src = render_target.surface.rect();
            let (size_x, size_y) = src.size();

            let dst_start = camera.transform.xform_inverse(pos).rounded();
            let dst_end = camera
                .transform
                .xform_inverse(Vector2F {
                    x: pos.x + size_x as f32,
                    y: pos.y + size_y as f32,
                })
                .rounded();

            let dst = Rect::new(
                dst_start.x,
                dst_start.y,
                (dst_end.x - dst_start.x) as u32,
                (dst_end.y - dst_start.y) as u32,
            );
            renderer::draw_surface(&mut canvas, &render_target.surface, src, dst)
        }
        renderer::finish_draw(&mut canvas);
    }
}
