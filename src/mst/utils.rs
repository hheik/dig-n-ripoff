use crate::util::{math::wrapping_quotient, Vector2I};

use super::chunk::Chunk;

pub fn global_to_local(position: &Vector2I) -> Vector2I {
    Vector2I {
        x: position.x % Chunk::SIZE.x,
        y: position.y % Chunk::SIZE.y,
    }
}

pub fn global_to_index(position: &Vector2I) -> Vector2I {
    let centered = *position + Chunk::SIZE / 2;
    Vector2I {
        x: wrapping_quotient(centered.x, Chunk::SIZE.x),
        y: wrapping_quotient(centered.y, Chunk::SIZE.y),
    }
}

pub fn index_to_global(ci: &Vector2I) -> Vector2I {
    *ci * Chunk::SIZE
}
