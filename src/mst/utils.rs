use crate::{components::Chunk, util::Vector2I};

pub fn global_to_chunk_index(pos: &Vector2I) -> Vector2I {
    Vector2I {
        x: wrapping_quotient(pos.x, Chunk::SIZE_X as i32),
        y: wrapping_quotient(pos.y, Chunk::SIZE_Y as i32),
    }
}

pub fn chunk_index_to_global(ci: &Vector2I) -> Vector2I {
    Vector2I {
        x: ci.x * Chunk::SIZE_X as i32,
        y: ci.y * Chunk::SIZE_Y as i32,
    }
}

/// Calculate quotient, but take into account negative values so that they continue the cycle seamlessly.
/// e.g. (0, 4) -> 0, (-4, 4) -> -1, (-5, 4) -> -2
fn wrapping_quotient(dividend: i32, divisor: i32) -> i32 {
    let res = (if dividend < 0 { dividend + 1 } else { dividend }) / divisor;
    if dividend < 0 {
        res - 1
    } else {
        res
    }
}
