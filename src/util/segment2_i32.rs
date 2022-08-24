use super::Vector2I;
use std::fmt;

pub struct Segment2I {
    from: Vector2I,
    to: Vector2I,
}

impl fmt::Display for Segment2I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
