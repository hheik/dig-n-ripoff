use std::fmt;
use std::ops;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vector2I {
    pub x: i32,
    pub y: i32,
}

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

impl fmt::Display for Vector2I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add<Vector2I> for Vector2I {
    type Output = Vector2I;
    fn add(self, rhs: Vector2I) -> Self::Output {
        Vector2I {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Neg for Vector2I {
    type Output = Vector2I;
    fn neg(self) -> Self::Output {
        Vector2I {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Sub<Vector2I> for Vector2I {
    type Output = Vector2I;
    fn sub(self, rhs: Vector2I) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul<Vector2I> for Vector2I {
    type Output = Vector2I;
    fn mul(self, rhs: Vector2I) -> Self::Output {
        Vector2I {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<i32> for Vector2I {
    type Output = Vector2I;
    fn mul(self, rhs: i32) -> Self::Output {
        Vector2I {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector2I> for i32 {
    type Output = Vector2I;
    fn mul(self, rhs: Vector2I) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<Vector2I> for Vector2I {
    type Output = Vector2I;
    fn div(self, rhs: Vector2I) -> Self::Output {
        Vector2I {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<i32> for Vector2I {
    type Output = Vector2I;
    fn div(self, rhs: i32) -> Self::Output {
        Vector2I {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
