use sdl2::pixels::Color;
use specs::{Component, VecStorage};

use crate::util::Vector2I;

#[derive(Component)]
#[storage(VecStorage)]
pub struct ElementShadow {
    pub color: Color,
    pub offset: Vector2I,
}

impl ElementShadow {
    pub const DEFAULT_SHADOW_COLOR: Color = Color {
        r: 25,
        g: 20,
        b: 20,
        a: 255,
    };

    pub fn new() -> Self {
        ElementShadow {
            color: Self::DEFAULT_SHADOW_COLOR,
            offset: Vector2I { x: 1, y: 1 },
        }
    }
}
