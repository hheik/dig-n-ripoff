use specs::{Component, VecStorage};

use crate::{
    resources::UnsafeBody,
    util::{
        box2d::{b2vec_to_vector2f, vector2f_to_b2vec},
        Vector2F,
    },
};

pub struct PhysicsBody {
    body: UnsafeBody,
}

impl PhysicsBody {
    pub fn new(body: UnsafeBody) -> PhysicsBody {
        PhysicsBody { body }
    }

    pub fn get_position(&self) -> Vector2F {
        b2vec_to_vector2f(self.body.borrow().get_position())
    }

    pub fn get_rotation(&self) -> f32 {
        self.body.borrow().get_angle()
    }

    pub fn get_linear_velocity(&self) -> Vector2F {
        b2vec_to_vector2f(self.body.borrow().get_linear_velocity())
    }

    // FIXME: Panics
    /// Doesn't work, don't use
    pub fn set_transform(&mut self, position: Option<Vector2F>, rotation: Option<f32>) {
        let pos = match position {
            Some(position) => vector2f_to_b2vec(position),
            None => self.body.borrow().get_position(),
        };
        let rot = rotation.unwrap_or(self.body.borrow().get_angle());
        self.body.borrow_mut().set_transform(pos, rot);
    }

    // FIXME: Panics probably
    /// Doesn't work, don't use
    pub fn set_linear_velocity(&self, value: Vector2F) {
        self.body
            .borrow_mut()
            .set_linear_velocity(vector2f_to_b2vec(value));
    }
}

impl Component for PhysicsBody {
    type Storage = VecStorage<Self>;
}
