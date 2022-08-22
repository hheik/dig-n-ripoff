pub use u8 as TexelID;

// use crate::util::Vector2I;

// pub struct TexelPosition {
//     pub texel: Texel,
//     pub pos: Vector2I,
// }

#[derive(Clone, Copy)]
pub struct Texel {
    pub id: TexelID,
}

impl Texel {
    pub fn is_empty(&self) -> bool {
        self.id == 0
    }

    pub fn empty() -> Texel {
        Texel { id: 0 }
    }
}
