use box2d_rs::{b2_body::{B2bodyDef}, b2_world::{B2worldPtr, B2world}};
use specs::{Component, VecStorage};
use unsafe_send_sync::UnsafeSendSync;

use crate::resources::{UserData, UnsafeBody};

pub struct PhysicsBody {
    body: UnsafeBody
}

impl PhysicsBody {
    pub fn new(world: B2worldPtr<UserData>) -> PhysicsBody {
        let mut body: B2bodyDef<UserData> = B2bodyDef::default();
        let body: UnsafeBody = UnsafeSendSync::new(B2world::create_body(world.clone(), &body));
        PhysicsBody {
            body
        }
    }
}

impl Component for PhysicsBody {
    type Storage = VecStorage<Self>;
}
