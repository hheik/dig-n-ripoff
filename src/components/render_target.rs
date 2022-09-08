use sdl2::{pixels::Color, surface::Surface};
use specs::{Component, VecStorage};

use crate::{
    gl::renderer::{UnsafeSurface, SURFACE_FORMAT},
    util::Vector2F,
};

pub struct RenderTarget<'a> {
    pub surface: UnsafeSurface<'a>,
    pub pivot: Vector2F,
    pub sorting_order: i16,
    pub use_screen_space: bool,
}

impl<'a> RenderTarget<'a> {
    pub fn new(
        width: u32,
        height: u32,
        pivot: Vector2F,
        sorting_order: i16,
        use_screen_space: bool,
    ) -> Self {
        RenderTarget {
            surface: UnsafeSurface::new(Surface::new(width, height, SURFACE_FORMAT).unwrap()),
            pivot,
            sorting_order,
            use_screen_space,
        }
    }

    pub fn new_filled(
        width: u32,
        height: u32,
        pivot: Vector2F,
        sorting_order: i16,
        use_screen_space: bool,
        (r, g, b, a): (u8, u8, u8, u8),
    ) -> Self {
        let mut render_target = Self::new(width, height, pivot, sorting_order, use_screen_space);
        let rect = render_target.surface.rect();
        match render_target
            .surface
            .fill_rect(rect, Color::RGBA(r, g, b, a))
        {
            Ok(_) => (),
            Err(error) => panic!("Failed to create filled render target: {error:?}"),
        }
        render_target
    }

    pub fn set_surface(&mut self, surface: Surface<'a>) {
        self.surface = UnsafeSurface::new(surface);
    }
}

impl Component for RenderTarget<'static> {
    type Storage = VecStorage<Self>;
}
