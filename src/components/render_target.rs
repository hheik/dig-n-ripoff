use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct RenderTarget {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub is_dirty: bool,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> RenderTarget {
        let mut buffer: Vec<u32> = Vec::new();
        buffer.resize(width * height, 0);
        RenderTarget {
            width,
            height,
            buffer,
            is_dirty: true,
        }
    }
}
