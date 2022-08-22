use specs::{Component, VecStorage};

use crate::util::Vector2;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32,
    pub scale: Vector2<f32>,
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            position: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            scale: Vector2 { x: 1.0, y: 1.0 },
        }
    }

    pub fn new(position: Vector2<f32>) -> Transform {
        Transform {
            position,
            rotation: 0.0,
            scale: Vector2 { x: 1.0, y: 1.0 },
        }
    }
}
