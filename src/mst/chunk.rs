use crate::{
    mst::texel::{Texel, TexelID, NEIGHBOUR_INDEX_MAP},
    util::{ChangeBuffer, Vector2I},
};
use specs::{Component, DenseVecStorage};

#[derive(Clone, Copy)]
pub struct TexelUpdate {
    pub global_position: Vector2I,
    pub id: TexelID,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Chunk {
    pub texels: Box<[Texel; (Self::SIZE_X * Self::SIZE_Y) as usize]>,
    change_buffer: ChangeBuffer<TexelUpdate>,
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
            change_buffer: ChangeBuffer::new(),
        }
    }

    pub fn new_texel_array() -> Box<[Texel; Self::SIZE_X * Self::SIZE_Y]> {
        Box::new([Texel::default(); Self::SIZE_X * Self::SIZE_Y])
    }

    pub fn get_texel(&self, position: &Vector2I) -> Texel {
        self.texels[position.y as usize * Chunk::SIZE_X + position.x as usize]
    }

    pub fn get_texel_option_mut(&mut self, position: &Vector2I) -> Option<&mut Texel> {
        if position.x < 0
            || position.y < 0
            || position.x >= Chunk::SIZE_X as i32
            || position.y >= Chunk::SIZE_Y as i32
        {
            return None;
        }
        Some(&mut self.texels[position.y as usize * Chunk::SIZE_X + position.x as usize])
    }

    pub fn set_texel(&mut self, position: &Vector2I, id: TexelID) {
        let i = position.y as usize * Chunk::SIZE_X + position.x as usize;
        if self.texels[i].id != id {
            // self.is_dirty = true;
        }
        let update_neighbours = self.texels[i].is_empty()
            != (Texel {
                id,
                neighbour_mask: 0,
            })
            .is_empty();
        self.texels[i].id = id;
        // Update neighbour mask
        if update_neighbours {
            for offset in Texel::NEIGHBOUR_OFFSET_VECTORS {
                // Flip neighbour's bit
                match self.get_texel_option_mut(&(*position + offset)) {
                    Some(mut neighbour) => {
                        neighbour.neighbour_mask ^= 1 << NEIGHBOUR_INDEX_MAP[&-offset];
                    }
                    None => (),
                }
            }
        }
    }

    // pub fn add_listener(&mut self, cb: impl Fn(&Vector2I, &TexelID) -> ()) {
    //     // self.on_change.push(cb);
    // }

    // pub fn add_listener<F>(&mut self, cb: F)
    // where
    //     F: Fn(&Vector2I, &TexelID) -> (),
    // {
    //     self.on_change.push(|local, id| cb(&local, &id));
    // }
}
