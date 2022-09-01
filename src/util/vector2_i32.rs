use core::ops;

use super::Vector2;

use sdl2::rect::Rect;

pub type Vector2I = Vector2<i32>;

impl Vector2I {
    pub const ZERO: Vector2I = Vector2I { x: 0, y: 0 };
    pub const ONE: Vector2I = Vector2I { x: 1, y: 1 };
    pub const UP: Vector2I = Vector2I { x: 0, y: -1 };
    pub const DOWN: Vector2I = Vector2I { x: 0, y: 1 };
    pub const LEFT: Vector2I = Vector2I { x: -1, y: 0 };
    pub const RIGHT: Vector2I = Vector2I { x: 1, y: 0 };

    pub fn angle(&self) -> f32 {
        (self.y as f32).atan2(self.x as f32)
    }
}

impl ops::Mul<Rect> for Vector2I {
    type Output = Rect;
    fn mul(self, rhs: Rect) -> Self::Output {
        Rect::new(
            rhs.x,
            rhs.y,
            rhs.width() * self.x as u32,
            rhs.height() * self.y as u32,
        )
    }
}
