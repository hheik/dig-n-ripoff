use specs::{System, Write, Read};

use crate::resources::{UnsafeBox2D, Time};

pub struct Box2DPhysics {}

impl Box2DPhysics {
    pub fn new() -> Box2DPhysics {
        Box2DPhysics {}
    }
}

impl<'a> System<'a> for Box2DPhysics {
    type SystemData = (Write<'a, UnsafeBox2D>, Read<'a, Time>);
    fn run(&mut self, (mut box2d, time): Self::SystemData) {
        
        let world = box2d.world_ptr.borrow_mut();
    }
}
