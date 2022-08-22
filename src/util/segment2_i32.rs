use super::Vector2I;
use std::fmt;

pub struct Segment2I {
    from: Vector2I,
    to: Vector2I,
}

impl Segment2I {
    pub fn diff(&self) -> Vector2I {
        self.to - self.from
    }

    pub fn angle(&self) -> f32 {
        self.diff().angle()
    }

    pub fn swapped(&self) -> Segment2I {
        Segment2I {
            from: self.to.clone(),
            to: self.from.clone(),
        }
    }
}

impl fmt::Display for Segment2I {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
