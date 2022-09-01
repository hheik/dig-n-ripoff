use super::Vector2I;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment2I {
    pub from: Vector2I,
    pub to: Vector2I,
}

impl Segment2I {
    pub fn diff(&self) -> Vector2I {
        self.to - self.from
    }

    pub fn angle(&self) -> f32 {
        self.diff().angle()
    }
}

impl fmt::Display for Segment2I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
