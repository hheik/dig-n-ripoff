use crate::{
    mst::{
        texel::TexelID,
        utils::{index_to_global, texel_index_to_local},
    },
    resources::{Terrain, Time},
    util::Vector2I,
};
use specs::{Read, System, Write};

pub struct TerrainPainter;
impl<'a> System<'a> for TerrainPainter {
    type SystemData = (Read<'a, Time>, Write<'a, Terrain>);

    fn run(&mut self, (time, mut terrain): Self::SystemData) {
        let mut updates: Vec<(Vector2I, TexelID)> = Vec::new();
        for (index, chunk) in terrain.chunk_iter() {
            for i in 0..chunk.texels.len() {
                let local = texel_index_to_local(i);
                let global = index_to_global(index) + local;
                // rng gen from crate rand was super slow, but even this is quite slow
                let rng = (time.lifetime.elapsed().unwrap().as_millis()
                    % (time.frame + 1000) as u128)
                    + (global.x * global.x + (global.y * 387)) as u128;
                if rng % 1000 < 5
                    && match terrain.global_to_texel(&global) {
                        Some(texel) => texel.is_empty(),
                        None => false,
                    }
                    && (match terrain.global_to_texel(&(global + Vector2I::UP)) {
                        Some(texel) => texel.id == 2,
                        None => false,
                    } || match terrain.global_to_texel(&(global + Vector2I::DOWN)) {
                        Some(texel) => texel.id == 2,
                        None => false,
                    } || match terrain.global_to_texel(&(global + Vector2I::LEFT)) {
                        Some(texel) => texel.id == 2,
                        None => false,
                    } || match terrain.global_to_texel(&(global + Vector2I::RIGHT)) {
                        Some(texel) => texel.id == 2,
                        None => false,
                    })
                {
                    updates.push((global.to_owned(), 2));
                }
            }
        }
        loop {
            match updates.pop() {
                Some((global, id)) => terrain.set_texel(&global, id),
                None => break,
            }
        }
    }
}
