use super::{chunk::Chunk, texel::TexelID};
use sdl2::{pixels::Color, surface::Surface};
use std::collections::HashMap;

pub struct ChunkRenderer {
    pub chunk: Chunk,
    pub surface: Surface<'static>,
}

impl ChunkRenderer {
    pub fn draw_to_surface(&mut self, bytes_per_pixel: usize) {
        let color_map: HashMap<TexelID, Color> = [
            (0, Color::RGBA(0, 0, 0, 0)),
            (1, Color::RGBA(158, 127, 99, 255)),
            (2, Color::RGBA(70, 142, 71, 255)),
        ]
        .iter()
        .cloned()
        .collect();

        self.surface.with_lock_mut(|p_data| {
            if p_data.len() != self.chunk.texels.len() * bytes_per_pixel {
                panic!("Surface pixel count is not aligned with texel count");
            }

            // TODO: This doesn't care about bytes_per_pixel
            for xy in 0..self.chunk.texels.len() {
                let i = xy * bytes_per_pixel;
                let c = color_map[&self.chunk.texels[xy].id];
                p_data[i + 0] = c.r;
                p_data[i + 1] = c.g;
                p_data[i + 2] = c.b;
                p_data[i + 3] = c.a;
                // p_data[i] = self.chunk.texels[xy].id;
            }
        })
    }
}
