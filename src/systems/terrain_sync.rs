use std::collections::HashSet;

use crate::{
    components::{ChunkIndex, PhysicsBody, RenderTarget, Transform},
    mst::{chunk::Chunk, marching_square, utils::index_to_global},
    resources::{Terrain, UnsafeBox2D},
    util::{
        box2d::{create_body, create_segmented_shape},
        SortingOrder, Vector2F, Vector2I,
    },
};

use box2d_rs::{b2_body::B2bodyType, shapes::b2_chain_shape::B2chainShape};
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
        Entities<'a>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, ChunkIndex>,
        WriteStorage<'a, RenderTarget<'static>>,
        WriteStorage<'a, PhysicsBody>,
        Read<'a, Terrain>,
        Read<'a, UnsafeBox2D>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transform,
            mut chunk_index,
            mut render_target,
            mut physics_body,
            terrain,
            box2d,
        ): Self::SystemData,
    ) {
        // Add new chunks
        for (index, chunk) in terrain.chunk_iter() {
            if !self.chunk_set.contains(index) {
                let transform_component =
                    Transform::new(Vector2F::from(index_to_global(index)), 0.0, Vector2F::ONE);

                let now = std::time::SystemTime::now();
                let islands = marching_square::calculate_collisions(chunk);
                println!(
                    "{}: collision generation took {}ms {} shapes",
                    index,
                    now.elapsed().unwrap().as_millis(),
                    islands.len()
                );
                let mut shapes: Vec<B2chainShape> = Vec::with_capacity(islands.len());
                for island in islands {
                    shapes.push(create_segmented_shape(island))
                }

                let body = PhysicsBody::new(create_body(
                    box2d.world_ptr.clone(),
                    Some(B2bodyType::B2StaticBody),
                    vec![],
                    shapes,
                    Some(transform_component.get_position()),
                    Some(transform_component.get_rotation()),
                ));

                entities
                    .build_entity()
                    .with(transform_component, &mut transform)
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
                            SortingOrder::Default as i16,
                            false,
                        ),
                        &mut render_target,
                    )
                    .with(body, &mut physics_body)
                    .build();
                self.chunk_set.insert(index.to_owned());
            }
        }

        // Remove deleted chunks
        self.chunk_set
            .retain(|index| terrain.index_to_chunk(index).is_some());
    }
}
