use specs::{Component, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct DebugText;
