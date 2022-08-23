use std::collections::HashMap;

use crate::{
    components::{Chunk, RenderTarget, Transform},
    gl::{camera::Camera, renderer::SURFACE_FORMAT},
    mst::texel::TexelID,
};
use sdl2::pixels::{Color, PixelFormat};
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct TerrainRender;
impl<'a> System<'a> for TerrainRender {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Chunk>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, RenderTarget>,
        Read<'a, Camera>,
    );

    fn run(&mut self, (entity, chunk, transform, mut render_target, camera): Self::SystemData) {
        let color_map: HashMap<TexelID, u32> = [
            (
                0,
                Color::RGBA(0, 0, 0, 0).to_u32(&PixelFormat::try_from(SURFACE_FORMAT).unwrap()),
            ),
            (
                1,
                Color::RGBA(158, 127, 99, 255)
                    .to_u32(&PixelFormat::try_from(SURFACE_FORMAT).unwrap()),
            ),
            (
                2,
                Color::RGBA(70, 142, 71, 255)
                    .to_u32(&PixelFormat::try_from(SURFACE_FORMAT).unwrap()),
            ),
        ]
        .iter()
        .cloned()
        .collect();

        println!("*** Running TerrainRender ***");
        for (entity, chunk, transform, render_target) in
            (&entity, &chunk, &transform, &mut render_target).join()
        {
            println!(
                "Entity {}.{} : {} [{}]",
                entity.id(),
                entity.gen().id(),
                transform.position,
                chunk.texels.len()
            );
            if render_target.is_dirty == true {
                render_target.buffer = chunk.texels.iter().map(|t| color_map[&t.id]).collect();
                render_target.is_dirty = false;
            }
        }
    }
}
