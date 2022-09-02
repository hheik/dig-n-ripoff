use std::{cell::RefCell, rc::Rc};

use box2d_rs::{
    b2_body::{B2body, B2bodyDef, B2bodyType, BodyPtr},
    b2_fixture::B2fixtureDef,
    b2_math::B2vec2,
    b2_world::{B2world, B2worldPtr},
    b2rs_common::UserDataType,
    shapes::{b2_polygon_shape::B2polygonShape, b2_chain_shape::B2chainShape},
};
use unsafe_send_sync::UnsafeSendSync;

use crate::util::{box2d::vector2f_to_b2vec, Vector2F};

pub type UnsafeBox2D = UnsafeSendSync<Box2D>;
pub type UnsafeBody = UnsafeSendSync<BodyPtr<UserData>>;

#[derive(Clone, Copy, Default)]
pub struct UserData;
impl UserDataType for UserData {
    type Body = Option<u32>;
    type Fixture = u32;
    type Joint = ();
}

pub struct Box2D {
    pub gravity: B2vec2,
    pub world_ptr: B2worldPtr<UserData>,
}

impl Box2D {
    pub const METERS_TO_TEXELS: f32 = 4.0;
    pub const TEXELS_TO_METERS: f32 = 1.0 / Self::METERS_TO_TEXELS;
    const INIT_POS: B2vec2 = B2vec2 {
        x: -1000.0,
        y: -1000.0,
    };

    fn new() -> Box2D {
        let gravity: B2vec2 = B2vec2 { x: 0.0, y: 100.0 };
        let world_ptr: B2worldPtr<UserData> = B2world::new(gravity);

        Box2D { gravity, world_ptr }
    }

    pub fn new_unsafe() -> UnsafeBox2D {
        UnsafeBox2D::new(Self::new())
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
            None => Self::INIT_POS,
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
        }

        UnsafeBody::new(body_ptr)
    }
}

impl Default for Box2D {
    fn default() -> Self {
        Self::new()
    }
}
