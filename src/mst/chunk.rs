use crate::{mst::texel::Texel, util::Vector2I};
use specs::{Component, DenseVecStorage};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Chunk {
    pub texels: Box<[Texel; (Self::SIZE_X * Self::SIZE_Y) as usize]>,
    pub is_dirty: bool,
}

impl Chunk {
    pub const SIZE_X: usize = 64;
    pub const SIZE_Y: usize = 64;
    pub const SIZE: Vector2I = Vector2I {
        x: Self::SIZE_X as i32,
        y: Self::SIZE_Y as i32,
    };

    pub fn new() -> Chunk {
        Chunk {
            texels: Self::new_texel_array(),
            is_dirty: true,
        }
    }

    pub fn new_texel_array() -> Box<[Texel; Self::SIZE_X * Self::SIZE_Y]> {
        Box::new([Texel::EMPTY; Self::SIZE_X * Self::SIZE_Y])
    }

    pub fn get_texel(&self, position: &Vector2I) -> Texel {
        self.texels[position.y as usize * Chunk::SIZE_X + position.x as usize]
    }

    pub fn set_texel(&mut self, position: &Vector2I, value: Texel) {
        let i = position.y as usize * Chunk::SIZE_X + position.x as usize;
        if self.texels[i].id != value.id {
            self.is_dirty = true;
        }
        self.texels[i] = value;
    }
}
