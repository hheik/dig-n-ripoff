use crate::{
    mst::chunk::Chunk,
    resources::{Terrain, Time},
};
use rand::{seq::SliceRandom, thread_rng, Rng};
use specs::{Read, System, Write};

pub struct TerrainPainter;
impl<'a> System<'a> for TerrainPainter {
    type SystemData = (Read<'a, Time>, Write<'a, Terrain>);

    fn run(&mut self, (time, mut terrain): Self::SystemData) {
        if time.lifetime.elapsed().unwrap().as_secs() < 1 {
            return;
        }
        for (position_index, chunk) in terrain.chunk_iter_mut() {
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
        }
    }
}
