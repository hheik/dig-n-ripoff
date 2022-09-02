use crate::{
    components::{RenderTarget, Transform},
    gl::renderer::{self, UnsafeCanvas},
    resources::Camera,
    util::Vector2F,
};
use sdl2::rect::{Point, Rect};
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
        let cam_transform = camera.transform.with_rotation(0.0);

        for (transform, render_target) in (&transform, &render_target).join() {
            let src = render_target.surface.rect();
            let (size_x, size_y) = src.size();
            let pivot_offset = render_target.pivot
                * Vector2F {
                    x: size_x as f32,
                    y: size_y as f32,
                };
            let pos = transform.get_position() - pivot_offset;

            let dst_start = cam_transform.xform_inverse(pos).rounded();
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
            // FIXME: Camera rotation is broken
            renderer::draw_surface_rotated(
                &mut canvas,
                &render_target.surface,
                src,
                dst,
                transform.get_rotation() as f64,
                Point::new(
                    size_x as i32 * (cam_transform.get_scale().x * render_target.pivot.x) as i32,
                    size_y as i32 * (cam_transform.get_scale().y * render_target.pivot.y) as i32,
                ),
                false,
                false,
            );
        }
    }
}
