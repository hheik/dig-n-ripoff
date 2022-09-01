use std::collections::HashMap;

use lazy_static::lazy_static;
pub use u8 as TexelID;
pub use u8 as NeighbourMask;

use crate::util::Vector2I;

use super::chunk::Chunk;

#[derive(Clone, Copy, Default)]
pub struct Texel {
    pub id: TexelID,
    /// bitmask of empty/non-empty neighbours, goes clockwise from top (least significant bit first).
    pub neighbour_mask: NeighbourMask,
}

lazy_static! {
    pub static ref NEIGHBOUR_INDEX_MAP: HashMap<Vector2I, u8> = {
        let mut map = HashMap::new();
        for i in 0..Texel::NEIGHBOUR_OFFSET_VECTORS.len() {
            map.insert(Texel::NEIGHBOUR_OFFSET_VECTORS[i], i as u8);
        }
        map
    };
}

impl Texel {
    pub const EMPTY: TexelID = 0;
    pub const NEIGHBOUR_OFFSET_VECTORS: [Vector2I; 8] = [
        Vector2I { x: -1, y: -1 },
        Vector2I { x:  0, y: -1 },
        Vector2I { x:  1, y: -1 },
        Vector2I { x: -1, y:  0 },
        Vector2I { x:  1, y:  0 },
        Vector2I { x: -1, y:  1 },
        Vector2I { x:  0, y:  1 },
        Vector2I { x:  1, y:  1 },
    ];
    pub const NEIGHBOUR_OFFSET_INDICES: [i32; 8] = [
        -(Chunk::SIZE_X as i32) - 1,
        -(Chunk::SIZE_X as i32),
        -(Chunk::SIZE_X as i32) + 1,
        -(Chunk::SIZE_X as i32) - 1,
        -(Chunk::SIZE_X as i32) + 1,
        -(Chunk::SIZE_X as i32) - 1,
        -(Chunk::SIZE_X as i32),
        -(Chunk::SIZE_X as i32) + 1,
    ];
    
    pub fn is_empty(&self) -> bool {
        self.id == 0
    }
}
