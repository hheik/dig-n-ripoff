use specs::{System, Write};

use crate::resources::UnsafeBox2D;

pub struct Box2DPhysics {}

impl Box2DPhysics {
    pub fn new() -> Box2DPhysics {
        Box2DPhysics {}
    }
}

impl<'a> System<'a> for Box2DPhysics {
    type SystemData = Write<'a, UnsafeBox2D>;
    fn run(&mut self, mut box2d: Self::SystemData) {
        let world = box2d.world_ptr.borrow_mut();
    }
}
