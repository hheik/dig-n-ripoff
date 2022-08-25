use core::ops;
use std::fmt::Display;

use specs::{Component, VecStorage};

use crate::util::Vector2F;

#[derive(Component, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Transform {
    /// Matrix consisting of x, y and basis (position)
    matrix: [f32; 6],
    is_x_flipped: bool,
    is_y_flipped: bool,
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        is_x_flipped: false,
        is_y_flipped: false,
    };

    pub fn new(position: Vector2F, rotation: f32, scale: Vector2F) -> Transform {
        let mut transform = Transform::IDENTITY;
        transform.set_position(position);
        transform.set_rotation(rotation);
        transform.set_scale(scale);
        println!("{}", transform);
        transform
    }

    pub fn inverse(&self) -> Transform {
        // NOTIMP
    }

    pub fn get_position(&self) -> Vector2F {
        self.get_origin_column()
    }

    pub fn set_position(&mut self, value: Vector2F) {
        self.set_origin_column(value)
    }

    pub fn get_rotation(&self) -> f32 {
        self.get_x_column().angle() * if self.is_x_flipped { -1.0 } else { 1.0 }
    }

    pub fn set_rotation(&mut self, value: f32) {
        let x_len = self.get_x_column().length();
        let y_len = self.get_y_column().length();

        let (x, y): (Vector2F, Vector2F) = (
            Vector2F {
                x: value.cos(),
                y: value.sin(),
            } * x_len,
            Vector2F {
                x: -value.sin(),
                y: value.cos(),
            } * y_len,
        );

        self.set_x_column(x * if self.is_x_flipped { -1.0 } else { 1.0 });
        self.set_y_column(y * if self.is_y_flipped { -1.0 } else { 1.0 });
    }

    pub fn get_scale(&self) -> Vector2F {
        Vector2F {
            x: self.get_x_column().length() * if self.is_x_flipped { -1.0 } else { 1.0 },
            y: self.get_y_column().length() * if self.is_y_flipped { -1.0 } else { 1.0 },
        }
    }

    pub fn set_scale(&mut self, value: Vector2F) {
        let should_flip_x = value.x.is_sign_negative() != self.is_x_flipped;
        let should_flip_y = value.y.is_sign_negative() != self.is_y_flipped;

        self.set_x_column(
            self.get_x_column().normalized()
                * value.x.abs()
                * if should_flip_x { -1.0 } else { 1.0 },
        );
        self.set_y_column(
            self.get_y_column().normalized()
                * value.y.abs()
                * if should_flip_y { -1.0 } else { 1.0 },
        );

        self.is_x_flipped = value.x.is_sign_negative();
        self.is_y_flipped = value.y.is_sign_negative();
    }

    /// Transform point from local space to global space (assuming transform is global)
    pub fn xform(&self, point: Vector2F) -> Vector2F {
        self.get_position() + (self.get_x_column() * point.x) + (self.get_y_column() * point.y)
    }

    /// Transform point from global space to local space (assuming transform is global)
    pub fn xform_inverse(&self, point: Vector2F) -> Vector2F {
        self.get_position() + (self.get_x_column() * point.x) + (self.get_y_column() * point.y)
    }

    fn get_x_column(&self) -> Vector2F {
        Vector2F {
            x: self.matrix[0],
            y: self.matrix[1],
        }
    }

    fn set_x_column(&mut self, value: Vector2F) {
        self.matrix[0] = value.x;
        self.matrix[1] = value.y;
    }

    fn get_y_column(&self) -> Vector2F {
        Vector2F {
            x: self.matrix[2],
            y: self.matrix[3],
        }
    }

    fn set_y_column(&mut self, value: Vector2F) {
        self.matrix[2] = value.x;
        self.matrix[3] = value.y;
    }

    fn get_origin_column(&self) -> Vector2F {
        Vector2F {
            x: self.matrix[4],
            y: self.matrix[5],
        }
    }

    fn set_origin_column(&mut self, value: Vector2F) {
        self.matrix[4] = value.x;
        self.matrix[5] = value.y;
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} flips: {}, {}",
            self.matrix, self.is_x_flipped, self.is_y_flipped
        )
    }
}

impl ops::Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self::Output {
        // NOTIMP
    }
}
