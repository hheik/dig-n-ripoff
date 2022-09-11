use crate::{
    components::{ui::ElementShadow, RenderTarget, Transform},
    gl::renderer::{self, UnsafeCanvas},
    resources::Camera,
    util::Vector2F,
};
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
};
use specs::{rayon::slice::ParallelSliceMut, Join, Read, ReadStorage, System, Write};

pub struct Render;
impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, RenderTarget<'static>>,
        ReadStorage<'a, ElementShadow>,
        Read<'a, Camera>,
        Option<Write<'a, UnsafeCanvas>>,
    );

    fn run(&mut self, (transform, render_target, shadow, camera, canvas): Self::SystemData) {
        let mut canvas = match canvas {
            Some(canvas) => canvas,
            None => return,
        };

        let mut surfaces: Vec<(&Transform, &RenderTarget, Option<&ElementShadow>)> =
            (&transform, &render_target, (&shadow).maybe())
                .join()
                .collect();
        surfaces.par_sort_by(|a, b| a.1.sorting_order.cmp(&b.1.sorting_order));

        for (transform, render_target, shadow) in surfaces {
            let cam_transform = if render_target.use_screen_space {
                Transform::IDENTITY.with_scale(Vector2F::ONE)
            } else {
                camera.transform.with_rotation(0.0)
            };

            let src = render_target.surface.rect();
            let (size_x, size_y) = src.size();
            let pivot_offset = render_target.pivot
                * Vector2F {
                    x: size_x as f32,
                    y: size_y as f32,
                };
            let pos = transform.get_position() - pivot_offset;

            let dst_start = cam_transform.xform_inverse(pos).rounded();
            let dst_end = cam_transform
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

            match shadow {
                Some(shadow) => {
                    renderer::draw_surface_rotated(
                        &mut canvas,
                        &render_target.surface,
                        shadow.color,
                        src,
                        Rect::new(
                            dst.x + shadow.offset.x,
                            dst.y + shadow.offset.y,
                            dst.width(),
                            dst.height(),
                        ),
                        transform.get_rotation() as f64,
                        Point::new(
                            size_x as i32
                                * (cam_transform.get_scale().x * render_target.pivot.x) as i32,
                            size_y as i32
                                * (cam_transform.get_scale().y * render_target.pivot.y) as i32,
                        ),
                        transform.get_scale().x < 0.0,
                        transform.get_scale().y < 0.0,
                    );
                }
                None => (),
            }

            // FIXME: Camera rotation is broken
            renderer::draw_surface_rotated(
                &mut canvas,
                &render_target.surface,
                Color::WHITE,
                src,
                dst,
                transform.get_rotation() as f64,
                Point::new(
                    size_x as i32 * (cam_transform.get_scale().x * render_target.pivot.x) as i32,
                    size_y as i32 * (cam_transform.get_scale().y * render_target.pivot.y) as i32,
                ),
                transform.get_scale().x < 0.0,
                transform.get_scale().y < 0.0,
            );
        }
    }
}
