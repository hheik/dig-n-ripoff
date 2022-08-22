use specs::{Join, ReadStorage, System};

use crate::components::Chunk;

pub struct TerrainRender;

impl<'a> System<'a> for TerrainRender {
    type SystemData = ReadStorage<'a, Chunk>;

    fn run(&mut self, chunk: Self::SystemData) {
        for chunk in chunk.join() {
            println!("Chunk length: {}", &chunk.texels.len());
        }
    }
}
