use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{PhysicsBody, Transform},
    resources::{Time, UnsafeBox2D},
};

pub struct Box2DPhysics {
    phys_step_carry_over: f32,
}

impl Box2DPhysics {
    pub const TIME_STEP: f32 = 1.0 / 60.0;
    const VELOCITY_ITERATIONS: i32 = 6;
    const POSITION_ITERATIONS: i32 = 2;
    const MAX_PHYS_STEPS: u16 = 10;

    pub fn new() -> Box2DPhysics {
        Box2DPhysics {
            phys_step_carry_over: 0.0,
        }
    }
}

impl<'a> System<'a> for Box2DPhysics {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, PhysicsBody>,
        Write<'a, UnsafeBox2D>,
        Read<'a, Time>,
    );
    fn run(&mut self, (mut transform, physics_body, mut box2d, time): Self::SystemData) {
        let mut world = box2d.world_ptr.borrow_mut();
        // Perform a single, multiple, or no physics steps as needed
        // Make sure this won't send the engine in a cascade
        self.phys_step_carry_over += time.delta_time.as_secs_f32();
        let mut step_count = 0;
        for i in 0..Self::MAX_PHYS_STEPS {
            if self.phys_step_carry_over < 0.0 {
                break;
            }
            step_count = i.clone() + 1;
            world.step(
                Self::TIME_STEP,
                Self::VELOCITY_ITERATIONS,
                Self::POSITION_ITERATIONS,
            );
            self.phys_step_carry_over -= Self::TIME_STEP;
        }
        // Print if frame had many steps
        if step_count >= Self::MAX_PHYS_STEPS {
            println!(
                "frame: {} | steps: {} | fps: {}",
                time.frame,
                step_count,
                (1.0 / time.delta_time.as_secs_f32()).round()
            );
        }

        // Update transforms
        for (transform, physics_body) in (&mut transform, &physics_body).join() {
            transform.set_position(physics_body.get_position());
            transform.set_rotation(physics_body.get_rotation());
        }
    }
}
