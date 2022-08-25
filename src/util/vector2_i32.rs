use core::ops;

use super::Vector2;

use sdl2::rect::Rect;

pub type Vector2I = Vector2<i32>;

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
