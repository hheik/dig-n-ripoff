use crate::{components::Chunk, resources::Time, util::chunk_storage::ChunkWriteStorage};
use rand::{seq::SliceRandom, thread_rng, Rng};
use specs::{Read, System};

pub struct TerrainPainter;
impl<'a> System<'a> for TerrainPainter {
    type SystemData = (Read<'a, Time>, ChunkWriteStorage<'a>);

    fn run(&mut self, (time, mut chunk): Self::SystemData) {
        if time.lifetime.elapsed().unwrap().as_secs() < 1 {
            return;
        }
        for (chunk, transform) in (&mut chunk).join() {
            for i in 0..chunk.texels.len() {
                let rng: f64 = thread_rng().gen();
                if rng < 0.05
                    && chunk.texels[i].is_empty()
                    && i >= Chunk::SIZE_X
                    && i < (chunk.texels.len() - Chunk::SIZE_X)
                    && i % Chunk::SIZE_X != 0
                    && i % Chunk::SIZE_X != Chunk::SIZE_X - 1
                    && (chunk.texels[i - Chunk::SIZE_X].id == 2
                        || chunk.texels[i + Chunk::SIZE_X].id == 2
                        || chunk.texels[i - 1].id == 2
                        || chunk.texels[i + 1].id == 2)
                {
                    let mut indices: [usize; 4] =
                        [i - 1, i + 1, i - Chunk::SIZE_X, i + Chunk::SIZE_X];
                    indices.shuffle(&mut thread_rng());
                    for neighbour in indices.iter() {
                        if !chunk.texels[*neighbour].is_empty() {
                            chunk.texels[i].id = chunk.texels[*neighbour].id;
                            break;
                        }
                    }
                }
            }
            // for i in 0..chunk.texels.len() {
            //     let pos = Vector2I {
            //         x: (i % Chunk::SIZE_X) as i32,
            //         y: (i / Chunk::SIZE_X) as i32,
            //     } + ChunkWriteStorage::global_to_chunk_index(
            //         transform.get_position().rounded(),
            //     );
            //     let rng: f64 = thread_rng().gen();
            //     if rng < 0.05
            //         && ((data.get_texel(pos + Vector2I::UP).is_some()
            //             && data.get_texel(pos + Vector2I::UP).unwrap().id == 2)
            //             || (data.get_texel(pos + Vector2I::DOWN).is_some()
            //                 && data.get_texel(pos + Vector2I::DOWN).unwrap().id == 2)
            //             || (data.get_texel(pos + Vector2I::LEFT).is_some()
            //                 && data.get_texel(pos + Vector2I::LEFT).unwrap().id == 2)
            //             || (data.get_texel(pos + Vector2I::RIGHT).is_some()
            //                 && data.get_texel(pos + Vector2I::RIGHT).unwrap().id == 2))
            //     {
            //         data.set_texel(pos, Texel { id: 2 });
            //     }
            // }
        }
    }
}
