use std::collections::HashSet;

use crate::{
    components::{ChunkIndex, RenderTarget, Transform, PhysicsBody},
    mst::{chunk::Chunk, utils::index_to_global},
    resources::Terrain,
    util::{Vector2F, Vector2I},
};

use specs::{Entities, Read, System, WriteStorage};

pub struct TerrainSync {
    chunk_set: HashSet<Vector2I>,
}

impl TerrainSync {
    pub fn new() -> TerrainSync {
        TerrainSync {
            chunk_set: HashSet::new(),
        }
    }
}

impl<'a> System<'a> for TerrainSync {
    type SystemData = (
        Read<'a, Terrain>,
        Entities<'a>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, ChunkIndex>,
        WriteStorage<'a, RenderTarget<'static>>,
        WriteStorage<'a, PhysicsBody>,
    );

    fn run(
        &mut self,
        (terrain, entities, mut transform, mut chunk_index, mut render_target, mut physics_body): Self::SystemData,
    ) {
        // Add new chunks
        for (index, _) in terrain.chunk_iter() {
            if !self.chunk_set.contains(index) {
                entities
                    .build_entity()
                    .with(
                        Transform::new(Vector2F::from(index_to_global(index)), 0.0, Vector2F::ONE),
                        &mut transform,
                    )
                    .with(
                        ChunkIndex {
                            index: index.to_owned(),
                        },
                        &mut chunk_index,
                    )
                    .with(
                        RenderTarget::new(
                            Chunk::SIZE.x as u32,
                            Chunk::SIZE.y as u32,
                            Vector2F::ZERO,
                        ),
                        &mut render_target,
                    )
                    // .with(, &mut physics_body)
                    .build();
                self.chunk_set.insert(index.to_owned());
            }
        }
        // Remove deleted chunks
        self.chunk_set
            .retain(|index| terrain.index_to_chunk(index).is_some())
    }
}
