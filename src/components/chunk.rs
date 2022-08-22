use crate::mst::texel::Texel;
use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Chunk {
    pub texels: [Texel; (Self::SIZE_X * Self::SIZE_Y) as usize],
    pub is_dirty: bool,
}

impl Chunk {
    pub const SIZE_X: usize = 64;
    pub const SIZE_Y: usize = 64;

    pub fn new() -> Chunk {
        Chunk {
            texels: Self::new_texel_array(),
            is_dirty: false,
        }
    }

    pub fn new_texel_array() -> [Texel; Self::SIZE_X * Self::SIZE_Y] {
        [Texel::empty(); Self::SIZE_X * Self::SIZE_Y]
    }
}
