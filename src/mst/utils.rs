use crate::util::{
    math::{wrapping_quotient, wrapping_remainder},
    Vector2I,
};

use super::chunk::Chunk;

pub fn local_to_texel_index(position: &Vector2I) -> Option<usize> {
    match position.x >= 0
        && position.y >= 0
        && position.x < Chunk::SIZE.x
        && position.y < Chunk::SIZE.y
    {
        true => Some(position.y as usize * Chunk::SIZE_X + position.x as usize),
        false => None,
    }
}

pub fn texel_index_to_local(i: usize) -> Vector2I {
    Vector2I {
        x: i as i32 % Chunk::SIZE.x,
        y: i as i32 / Chunk::SIZE.y,
    }
}

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
