use sdl2::surface::Surface;
use specs::{Component, VecStorage};

use crate::gl::renderer::{UnsafeSurface, SURFACE_FORMAT};

pub struct RenderTarget<'a> {
    pub surface: UnsafeSurface<'a>,
    pub is_dirty: bool,
}

impl<'a> RenderTarget<'a> {
    pub fn new(width: usize, height: usize) -> RenderTarget<'a> {
        RenderTarget {
            surface: UnsafeSurface::new(
                Surface::new(width as u32, height as u32, SURFACE_FORMAT).unwrap(),
            ),
            is_dirty: true,
        }
    }
}

impl Component for RenderTarget<'static> {
    type Storage = VecStorage<Self>;
}
