use super::texel::Texel;
use crate::util::Vector2I;

pub const CHUNK_SIZE_X: usize = 64;
pub const CHUNK_SIZE_Y: usize = 64;

pub struct Chunk {
    pub position_index: Vector2I,
    pub texels: Box<[Texel; (CHUNK_SIZE_X * CHUNK_SIZE_Y) as usize]>,
}

impl Chunk {
    pub fn new(position_index: Vector2I) -> Chunk {
        Chunk {
            position_index,
            texels: Self::new_texel_array(),
        }
    }

    pub fn new_texel_array() -> Box<[Texel; CHUNK_SIZE_X * CHUNK_SIZE_Y]> {
        Box::new([Texel::empty(); CHUNK_SIZE_X * CHUNK_SIZE_Y])
    }
}
