use crate::util::Vector2I;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub use u8 as TexelID;
pub use u8 as NeighbourMask;

#[derive(Clone, Copy, Default)]
pub struct Texel {
    pub id: TexelID,
    /// bitmask of empty/non-empty neighbours, see NEIGHBOUR_OFFSET_VECTORS for the order
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
    pub const NEIGHBOUR_OFFSET_VECTORS: [Vector2I; 4] = [
        Vector2I { x: 0, y: -1 },
        Vector2I { x: 1, y: 0 },
        Vector2I { x: 0, y: 1 },
        Vector2I { x: -1, y: 0 },
    ];

    pub fn is_empty(&self) -> bool {
        self.id == 0
    }
}
