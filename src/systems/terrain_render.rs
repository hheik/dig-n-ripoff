use std::collections::HashMap;

use crate::{
    components::{ChunkIndex, RenderTarget},
    gl::renderer::SURFACE_FORMAT_BPP,
    mst::texel::TexelID,
    resources::{Terrain, TerrainUpdate},
    util::Listener,
};
use sdl2::pixels::Color;
use specs::{Join, ReadStorage, System, Write, WriteStorage};

pub struct TerrainRender {
    terrain_listener: Option<Listener>,
}

impl TerrainRender {
    pub fn new() -> TerrainRender {
        TerrainRender {
            terrain_listener: None,
        }
    }
}

// TODO: Find out why updating chunks cause permanent fps drops, or if terrain renderer is even the problem.
// Reproduce steps: rapidly paint/erase the same spot, eventually the idle fps just declines.
// The fps seems stable if terrain render is disabled
impl<'a> System<'a> for TerrainRender {
    type SystemData = (
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, RenderTarget<'static>>,
        Write<'a, Terrain>,
    );

    fn run(&mut self, (chunk, mut render_target, mut terrain): Self::SystemData) {
        let color_map: HashMap<TexelID, (u8, u8, u8, u8)> = [
            (0, Color::RGBA(0, 0, 0, 0).rgba()),
            (1, Color::RGBA(158, 127, 99, 255).rgba()),
            (2, Color::RGBA(70, 142, 71, 255).rgba()),
        ]
        .iter()
        .cloned()
        .collect();

        let events = match self.terrain_listener {
            Some(listener) => terrain.consume_changes(listener),
            None => {
                // Initialize
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
                None
            }
        };
        match events {
            Some(events) => {
                // Handle updates
                for event in events {
                    match event {
                        TerrainUpdate::ChunkAdded(index) => {
                            let (_, render_target) = match (&chunk, &mut render_target)
                                .join()
                                .find(|(chunk, _)| chunk.index == index)
                            {
                                Some(value) => value,
                                None => panic!("Could not find chunk entity for update"),
                            };
                            let chunk = match terrain.index_to_chunk(&index) {
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
                        TerrainUpdate::ChunkRemoved(index) => {}
                        TerrainUpdate::TexelsUpdated(index, changes) => {
                            let (_, render_target) = match (&chunk, &mut render_target)
                                .join()
                                .find(|(chunk, _)| chunk.index == index)
                            {
                                Some(value) => value,
                                None => panic!("Could not find chunk entity for update"),
                            };
                            let chunk = match terrain.index_to_chunk(&index) {
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
                        TerrainUpdate::None => (),
                    }
                }
            }
            None => (),
        };

        self.terrain_listener = Some(terrain.get_listener());
    }
}
