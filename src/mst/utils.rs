use crate::{components::Chunk, util::Vector2I};

pub fn chunk_index_to_global(ci: &Vector2I) -> Vector2I {
    Vector2I {
        x: ci.x * Chunk::SIZE_X as i32,
        y: ci.y * Chunk::SIZE_Y as i32,
    }
}
