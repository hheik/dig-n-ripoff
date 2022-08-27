use crate::util::Vector2I;
use specs::{Component, VecStorage};

#[derive(Component, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct ChunkIndex {
    pub index: Vector2I,
}
