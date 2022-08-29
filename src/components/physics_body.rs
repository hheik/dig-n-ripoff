use specs::{Component, VecStorage};

pub struct PhysicsBody {}

impl PhysicsBody {
    pub fn new() -> PhysicsBody {
        PhysicsBody {}
    }
}

impl Component for PhysicsBody {
    type Storage = VecStorage<Self>;
}
