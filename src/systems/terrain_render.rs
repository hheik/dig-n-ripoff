use std::collections::HashMap;

use crate::{
    components::{ChunkIndex, RenderTarget},
    gl::renderer::SURFACE_FORMAT_BPP,
    mst::texel::TexelID,
    resources::Terrain,
};
use sdl2::pixels::Color;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct TerrainRender;
impl<'a> System<'a> for TerrainRender {
    type SystemData = (
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, RenderTarget<'static>>,
        Read<'a, Terrain>,
    );

    fn run(&mut self, (chunk, mut render_target, terrain): Self::SystemData) {
        let color_map: HashMap<TexelID, (u8, u8, u8, u8)> = [
            (0, Color::RGBA(0, 0, 0, 0).rgba()),
            (1, Color::RGBA(158, 127, 99, 255).rgba()),
            (2, Color::RGBA(70, 142, 71, 255).rgba()),
        ]
        .iter()
        .cloned()
        .collect();

        for (chunk, render_target) in (&chunk, &mut render_target).join() {
            let chunk = match terrain.index_to_chunk(&chunk.index) {
                Some(chunk) => chunk,
                None => continue,
            };
            render_target.surface.with_lock_mut(|p_data| {
                assert!(p_data.len() == chunk.texels.len() * SURFACE_FORMAT_BPP);
                // FIXME: This doesn't care about bytes_per_pixel
                for xy in 0..chunk.texels.len() {
                    let i = xy * SURFACE_FORMAT_BPP;
                    let (r, g, b, a) = color_map[&chunk.texels[xy].id];
                    p_data[i + 0] = r;
                    p_data[i + 1] = g;
                    p_data[i + 2] = b;
                    p_data[i + 3] = a;
                }
            })
        }
    }
}
