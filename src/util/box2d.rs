use std::{cell::RefCell, rc::Rc};

use box2d_rs::{
    b2_body::{B2body, B2bodyDef, B2bodyType},
    b2_fixture::B2fixtureDef,
    b2_math::B2vec2,
    b2_world::{B2world, B2worldPtr},
    shapes::{b2_chain_shape::B2chainShape, b2_polygon_shape::B2polygonShape},
};
use specs::{Builder, World, WorldExt};

use crate::{
    components::{PhysicsBody, RenderTarget, Transform},
    resources::{Box2D, UnsafeBody, UserData},
};

use super::{SortingOrder, Vector2F};

pub fn b2vec_to_vector2f(value: B2vec2) -> Vector2F {
    Vector2F {
        x: value.x,
        y: value.y,
    } * Box2D::METERS_TO_TEXELS
}

pub fn vector2f_to_b2vec(value: Vector2F) -> B2vec2 {
    B2vec2 {
        x: value.x * Box2D::TEXELS_TO_METERS,
        y: value.y * Box2D::TEXELS_TO_METERS,
    }
}

/// <strong>Note: this function is not fully implemented</strong>
///
/// Only accepts max 8 vertices, and only produces convex shapes.
pub fn create_solid_shape(points: Vec<Vector2F>) -> Vec<B2polygonShape> {
    // TODO: handle more than 8 vertices and convex shapes
    let mut polygons: Vec<B2polygonShape> = Vec::new();
    {
        let mut shape = B2polygonShape::default();
        let points: Vec<B2vec2> = points.iter().map(|p| vector2f_to_b2vec(*p)).collect();
        shape.set(&points[..]);
        polygons.push(shape);
    }
    polygons
}

pub fn create_segmented_shape(points: Vec<Vector2F>) -> B2chainShape {
    let mut shape = B2chainShape::default();
    let points: Vec<B2vec2> = points.iter().map(|p| vector2f_to_b2vec(*p)).collect();
    shape.create_loop(&points[..]);
    shape
}

pub fn create_box(
    specs_world: &mut World,
    box2d_world: B2worldPtr<UserData>,
    body_type: B2bodyType,
    position: Vector2F,
    rotation: f32,
    size: Vector2F,
    (r, g, b, a): (u8, u8, u8, u8),
) {
    let mut shape = B2polygonShape::default();
    shape.set(&[
        vector2f_to_b2vec(
            Vector2F {
                x: -size.x,
                y: -size.y,
            } / 2.0,
        ),
        vector2f_to_b2vec(
            Vector2F {
                x: size.x,
                y: -size.y,
            } / 2.0,
        ),
        vector2f_to_b2vec(
            Vector2F {
                x: size.x,
                y: size.y,
            } / 2.0,
        ),
        vector2f_to_b2vec(
            Vector2F {
                x: -size.x,
                y: size.y,
            } / 2.0,
        ),
    ]);
    specs_world
        .create_entity()
        .with(Transform::IDENTITY)
        .with(PhysicsBody::new(create_body(
            box2d_world.clone(),
            Some(body_type),
            vec![shape],
            vec![],
            Some(position),
            Some(rotation),
        )))
        .with(RenderTarget::new_filled(
            size.x.round() as u32,
            size.y.round() as u32,
            Vector2F::ONE * 0.5,
            SortingOrder::Default as i16,
            false,
            (r, g, b, a),
        ))
        .build();
}

pub fn create_body(
    world: B2worldPtr<UserData>,
    body_type: Option<B2bodyType>,
    solid_shapes: Vec<B2polygonShape>,
    segmented_shapes: Vec<B2chainShape>,
    position: Option<Vector2F>,
    rotation: Option<f32>,
) -> UnsafeBody {
    let mut body_def: B2bodyDef<UserData> = B2bodyDef::default();
    body_def.body_type = body_type.unwrap_or_default();
    body_def.position = match position {
        Some(position) => vector2f_to_b2vec(position),
        None => Box2D::INIT_POS,
    };
    body_def.angle = rotation.unwrap_or(0.0);
    let body_ptr = B2world::create_body(world, &body_def);

    for shape in solid_shapes {
        let mut fixture_def: B2fixtureDef<UserData> = B2fixtureDef::default();
        fixture_def.shape = Some(Rc::new(RefCell::new(shape)));
        match body_def.body_type {
            B2bodyType::B2StaticBody => {
                fixture_def.density = 0.0;
                fixture_def.friction = 0.3;
            }
            B2bodyType::B2KinematicBody => {
                fixture_def.density = 1.0;
                fixture_def.friction = 0.0;
            }
            B2bodyType::B2DynamicBody => {
                fixture_def.density = 1.0;
                fixture_def.density = 0.3;
            }
        }
        B2body::create_fixture(body_ptr.clone(), &fixture_def);
    }

    for shape in segmented_shapes {
        let mut fixture_def: B2fixtureDef<UserData> = B2fixtureDef::default();
        fixture_def.shape = Some(Rc::new(RefCell::new(shape)));
        match body_def.body_type {
            B2bodyType::B2StaticBody => {
                fixture_def.density = 0.0;
                fixture_def.friction = 0.3;
            }
            B2bodyType::B2KinematicBody => {
                fixture_def.density = 1.0;
                fixture_def.friction = 0.0;
            }
            B2bodyType::B2DynamicBody => {
                fixture_def.density = 1.0;
                fixture_def.density = 0.3;
            }
        }
        B2body::create_fixture(body_ptr.clone(), &fixture_def);
        B2body::set_angular_velocity(&mut (*body_ptr.clone()).borrow_mut(), 1.0);
    }

    UnsafeBody::new(body_ptr)
}

pub fn replace_shape(
    body_ptr: UnsafeBody,
    solid_shapes: Vec<B2polygonShape>,
    segmented_shapes: Vec<B2chainShape>,
) {
    // B2body::destroy_fixture(self_, fixture)
    let mut fixtures = Vec::new();
    {
        for fixture_ptr in B2body::get_fixture_list(&RefCell::borrow(&body_ptr))
            .clone()
            .iter()
        {
            fixtures.push(fixture_ptr);
        }
    }
    for fixture in fixtures {
        B2body::destroy_fixture(body_ptr.clone().i, fixture);
    }

    let body_type = body_ptr.borrow().get_type();

    for shape in solid_shapes {
        let mut fixture_def: B2fixtureDef<UserData> = B2fixtureDef::default();
        fixture_def.shape = Some(Rc::new(RefCell::new(shape)));
        match body_type {
            B2bodyType::B2StaticBody => {
                fixture_def.density = 0.0;
                fixture_def.friction = 0.3;
            }
            B2bodyType::B2KinematicBody => {
                fixture_def.density = 1.0;
                fixture_def.friction = 0.0;
            }
            B2bodyType::B2DynamicBody => {
                fixture_def.density = 1.0;
                fixture_def.density = 0.3;
            }
        }
        B2body::create_fixture(body_ptr.clone().i, &fixture_def);
    }

    for shape in segmented_shapes {
        let mut fixture_def: B2fixtureDef<UserData> = B2fixtureDef::default();
        fixture_def.shape = Some(Rc::new(RefCell::new(shape)));
        match body_type {
            B2bodyType::B2StaticBody => {
                fixture_def.density = 0.0;
                fixture_def.friction = 0.3;
            }
            B2bodyType::B2KinematicBody => {
                fixture_def.density = 1.0;
                fixture_def.friction = 0.0;
            }
            B2bodyType::B2DynamicBody => {
                fixture_def.density = 1.0;
                fixture_def.density = 0.3;
            }
        }
        B2body::create_fixture(body_ptr.clone().i, &fixture_def);
    }
}

#[cfg(test)]
mod tests {
    use box2d_rs::b2_math::B2vec2;

    use super::{b2vec_to_vector2f, vector2f_to_b2vec};
    use crate::{resources::Box2D, util::Vector2F};

    #[test]
    fn both_ways() {
        let a = Vector2F { x: 1.0, y: 2.0 };
        let b = vector2f_to_b2vec(a);
        let c = b2vec_to_vector2f(b);
        // comparing f32's? works on my machine :)
        assert_eq!(
            b,
            B2vec2 {
                x: 1.0 * Box2D::TEXELS_TO_METERS,
                y: 2.0 * Box2D::TEXELS_TO_METERS
            }
        );
        assert_eq!(a, c);
    }
}
