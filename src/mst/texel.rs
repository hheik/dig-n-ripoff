pub use u8 as TexelID;

#[derive(Clone, Copy)]
pub struct Texel {
    pub id: TexelID,
}

impl Texel {
    pub const EMPTY: Texel = Texel { id: 0 };

    pub fn is_empty(&self) -> bool {
        self.id == 0
    }
}
