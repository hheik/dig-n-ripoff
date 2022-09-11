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
    pub const INIT_POS: B2vec2 = B2vec2 {
        x: -1000.0,
        y: -1000.0,
    };

    fn new() -> Box2D {
        let gravity: B2vec2 = B2vec2 { x: 0.0, y: 100.0 };
        // let gravity: B2vec2 = B2vec2 { x: 0.0, y: 1.0 };
        // let gravity: B2vec2 = B2vec2 { x: 0.0, y: 0.0 };
        let world_ptr: B2worldPtr<UserData> = B2world::new(gravity);

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
