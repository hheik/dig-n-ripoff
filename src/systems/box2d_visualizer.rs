use std::cell::RefCell;

use box2d_rs::{
    b2_body::{B2body, B2bodyType},
    b2_fixture::B2fixture,
    b2_world::B2world,
};
use specs::{Read, System, Write};

use crate::{
    gl::renderer::{self, UnsafeCanvas},
    resources::{Camera, UnsafeBox2D},
    util::{box2d::b2vec_to_vector2f, Vector2I},
};

pub struct Box2DVisualizer;
impl<'a> System<'a> for Box2DVisualizer {
    type SystemData = (
        Read<'a, Camera>,
        Read<'a, UnsafeBox2D>,
        Option<Write<'a, UnsafeCanvas>>,
    );

    fn run(&mut self, (camera, box2d, canvas): Self::SystemData) {
        let mut canvas = match canvas {
            Some(canvas) => canvas,
            None => return,
        };

        for body_ptr in B2world::get_body_list(&RefCell::borrow(&box2d.world_ptr)).iter() {
            let body = &*RefCell::borrow(&body_ptr);
            let color = match body.get_type() {
                B2bodyType::B2StaticBody => (0, 255, 255, 128),
                B2bodyType::B2KinematicBody => (255, 0, 255, 255),
                B2bodyType::B2DynamicBody => (255, 60, 60, 255),
            };
            for fixture_ptr in B2body::get_fixture_list(&RefCell::borrow(&body_ptr)).iter() {
                let shape = B2fixture::get_shape(&RefCell::borrow(&fixture_ptr));
                match shape.as_chain() {
                    Some(chain) => {
                        let points: Vec<Vector2I> = chain
                            .m_vertices
                            .iter()
                            .map(|v| {
                                camera
                                    .transform
                                    .xform_inverse(b2vec_to_vector2f(body.get_world_point(*v)))
                                    .rounded()
                            })
                            .collect();
                        renderer::draw_line_loop(&mut canvas, points, Some(color))
                    }
                    None => (),
                }
                match shape.as_polygon() {
                    Some(polygon) => {
                        let points: Vec<Vector2I> = polygon
                            .m_vertices
                            .iter()
                            .map(|v| {
                                camera
                                    .transform
                                    .xform_inverse(b2vec_to_vector2f(body.get_world_point(*v)))
                                    .rounded()
                            })
                            .collect();
                        renderer::draw_line_loop(
                            &mut canvas,
                            points[0..polygon.m_count].to_vec(),
                            Some(color),
                        )
                    }
                    None => (),
                }
            }
        }
    }
}
