use specs::{Read, System, Write};

use crate::resources::{Time, UnsafeBox2D};

pub struct Box2DPhysics {}

impl Box2DPhysics {
    pub const TIME_STEP: f32 = 1.0 / 60.0;
    const VELOCITY_ITERATIONS: i32 = 6;
    const POSITION_ITERATIONS: i32 = 2;

    pub fn new() -> Box2DPhysics {
        Box2DPhysics {}
    }
}

impl<'a> System<'a> for Box2DPhysics {
    type SystemData = (Write<'a, UnsafeBox2D>, Read<'a, Time>);
    fn run(&mut self, (mut box2d, time): Self::SystemData) {
        let mut world = box2d.world_ptr.borrow_mut();
        world.step(
            Self::TIME_STEP,
            Self::VELOCITY_ITERATIONS,
            Self::POSITION_ITERATIONS,
        );
        println!("bodies:");
        for body in world.get_body_list().iter() {
            let body = body.borrow();
            println!(
                "    pos: ({}, {}) angle: {} velocity: ({}, {})",
                body.get_position().x,
                body.get_position().y,
                body.get_angle(),
                body.get_linear_velocity().x,
                body.get_linear_velocity().y
            );
        }
    }
}
