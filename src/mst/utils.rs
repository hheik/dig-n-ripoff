use crate::util::{
    math::{wrapping_quotient, wrapping_remainder},
    Vector2I,
};

use super::chunk::Chunk;

pub fn global_to_local(position: &Vector2I) -> Vector2I {
    Vector2I {
        x: wrapping_remainder(position.x, Chunk::SIZE.x),
        y: wrapping_remainder(position.y, Chunk::SIZE.y),
    }
}

pub fn global_to_index(position: &Vector2I) -> Vector2I {
    Vector2I {
        x: wrapping_quotient(position.x, Chunk::SIZE.x),
        y: wrapping_quotient(position.y, Chunk::SIZE.y),
    }
}

pub fn index_to_global(ci: &Vector2I) -> Vector2I {
    *ci * Chunk::SIZE
}
