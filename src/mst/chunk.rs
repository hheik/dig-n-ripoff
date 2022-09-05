use crate::{
    mst::texel::{Texel, TexelID, NEIGHBOUR_INDEX_MAP},
    util::{ChangeBuffer, Vector2I},
};
use specs::{Component, DenseVecStorage};

use super::utils::local_to_texel_index;

#[derive(Clone, Copy)]
pub struct TexelUpdate {
    pub position: Vector2I,
    pub id: TexelID,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Chunk {
    pub texels: Box<[Texel; (Self::SIZE_X * Self::SIZE_Y) as usize]>,
    pub change_buffer: ChangeBuffer<TexelUpdate>,
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

    pub fn get_texel(&self, position: &Vector2I) -> Option<Texel> {
        local_to_texel_index(position).map(|i| self.texels[i])
    }

    pub fn get_texel_option_mut(&mut self, position: &Vector2I) -> Option<&mut Texel> {
        local_to_texel_index(position).map(|i| &mut self.texels[i])
    }

    pub fn set_texel(&mut self, position: &Vector2I, id: TexelID) {
        let i = local_to_texel_index(position).expect("Texel index out of range");
        if self.texels[i].id != id {
            self.change_buffer.push_event(TexelUpdate {
                position: *position,
                id,
            })
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
}
