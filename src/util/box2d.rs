use box2d_rs::{
    b2_body::B2bodyType, b2_math::B2vec2, b2_world::B2worldPtr,
    shapes::{b2_polygon_shape::B2polygonShape, b2_chain_shape::B2chainShape}, b2_settings::B2_MAX_POLYGON_VERTICES,
};
use specs::{Builder, World, WorldExt};

use crate::{
    components::{PhysicsBody, RenderTarget, Transform},
    resources::{Box2D, UserData},
};

use super::Vector2F;

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
        .with(PhysicsBody::new(Box2D::create_body(
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
            (r, g, b, a),
        ))
        .build();
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
