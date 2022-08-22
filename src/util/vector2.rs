use core::{fmt, ops};

pub trait VectorComponent:
    Sized
    + Copy
    + fmt::Display
    + ops::Add<Output = Self>
    + ops::Neg<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
{
}

impl<T> VectorComponent for T where
    T: Sized
        + Copy
        + fmt::Display
        + ops::Neg<Output = T>
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
{
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vector2<T: VectorComponent> {
    pub x: T,
    pub y: T,
}

impl<T: VectorComponent> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: VectorComponent> ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: VectorComponent> ops::Neg for Vector2<T> {
    type Output = Vector2<T>;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: VectorComponent> ops::Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        self + (-rhs)
    }
}

impl<T: VectorComponent> ops::Mul<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: VectorComponent> ops::Mul<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: VectorComponent> ops::Div<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn div(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: VectorComponent> ops::Div<T> for Vector2<T> {
    type Output = Vector2<T>;
    fn div(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
