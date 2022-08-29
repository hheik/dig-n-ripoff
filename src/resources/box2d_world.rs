use box2d_rs::{
    b2_body::BodyPtr,
    b2_math::B2vec2,
    b2_world::{B2world, B2worldPtr},
    b2rs_common::UserDataType,
};
use unsafe_send_sync::UnsafeSendSync;

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

    fn new() -> Box2D {
        let gravity: B2vec2 = B2vec2 { x: 0.0, y: 100.0 };
        let world_ptr: B2worldPtr<UserData> = B2world::new(gravity);

        // // Static ground body
        // {
        //     let mut body_def: B2bodyDef<UserData> = B2bodyDef::default();
        //     body_def.position.set(
        //         128.0 * Self::TEXELS_TO_METERS,
        //         128.0 * Self::TEXELS_TO_METERS,
        //     );
        //     let body_ptr = B2world::create_body(world_ptr.clone(), &body_def);
        //     let mut shape = B2polygonShape::default();
        //     shape.set_as_box(64.0 * Self::TEXELS_TO_METERS, 8.0 * Self::TEXELS_TO_METERS);
        //     B2body::create_fixture_by_shape(body_ptr, Rc::new(RefCell::new(shape)), 0.0);
        // }

        // // Dynamic box body
        // {
        //     let mut body_def: B2bodyDef<UserData> = B2bodyDef::default();
        //     body_def.body_type = B2bodyType::B2DynamicBody;
        //     body_def.position.set(
        //         128.0 * Self::TEXELS_TO_METERS,
        //         80.0 * Self::TEXELS_TO_METERS,
        //     );
        //     let body_ptr = B2world::create_body(world_ptr.clone(), &body_def);
        //     let mut shape = B2polygonShape::default();
        //     shape.set_as_box(4.0 * Self::TEXELS_TO_METERS, 4.0 * Self::TEXELS_TO_METERS);
        //     let mut fixture_def: B2fixtureDef<UserData> = B2fixtureDef::default();
        //     fixture_def.shape = Some(Rc::new(RefCell::new(shape)));
        //     fixture_def.density = 1.0;
        //     fixture_def.friction = 0.3;
        //     B2body::create_fixture(body_ptr, &fixture_def);
        // }

        Box2D { gravity, world_ptr }
    }

    pub fn new_unsafe() -> UnsafeBox2D {
        UnsafeBox2D::new(Self::new())
    }
}

impl Default for Box2D {
    fn default() -> Self {
        Self::new()
    }
}
