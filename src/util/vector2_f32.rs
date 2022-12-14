use std::convert::From;

use super::{Vector2, Vector2I};

pub type Vector2F = Vector2<f32>;

impl Vector2F {
    pub const ZERO: Vector2F = Vector2F { x: 0.0, y: 0.0 };
    pub const ONE: Vector2F = Vector2F { x: 1.0, y: 1.0 };
    pub const UP: Vector2F = Vector2F { x: 0.0, y: -1.0 };
    pub const DOWN: Vector2F = Vector2F { x: 0.0, y: 1.0 };
    pub const LEFT: Vector2F = Vector2F { x: -1.0, y: 0.0 };
    pub const RIGHT: Vector2F = Vector2F { x: 1.0, y: 0.0 };

    pub fn rounded(&self) -> Vector2I {
        Vector2I {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vector2F {
        return *self / self.length();
    }
}

impl From<Vector2I> for Vector2F {
    fn from(v: Vector2I) -> Self {
        Vector2F {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}
