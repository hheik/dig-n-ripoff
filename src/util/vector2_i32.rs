use super::Vector2;

pub type Vector2I = Vector2<i32>;

impl Vector2I {
    /// Angle of vector in radians
    pub fn angle(&self) -> f32 {
        (self.y as f32).atan2(self.x as f32)
    }
    /// returns (0, 0)
    pub fn zero() -> Vector2I {
        Vector2I { x: 0, y: 0 }
    }
    /// returns (1, 1)
    pub fn one() -> Vector2I {
        Vector2I { x: 1, y: 1 }
    }
    /// returns (0, -1)
    pub fn up() -> Vector2I {
        Vector2I { x: 0, y: -1 }
    }
    /// returns (0, 1)
    pub fn down() -> Vector2I {
        Vector2I { x: 0, y: 1 }
    }
    /// returns (-1, 0)
    pub fn left() -> Vector2I {
        Vector2I { x: -1, y: 0 }
    }
    /// returns (1, 0)
    pub fn right() -> Vector2I {
        Vector2I { x: 1, y: 0 }
    }
}
