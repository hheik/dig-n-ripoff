use crate::{
    components::{ChunkIndex, PhysicsBody, Transform},
    mst::marching_square,
    resources::{Terrain, TerrainUpdate, UnsafeBox2D},
    util::{
        box2d::{create_segmented_shape, replace_shape},
        Listener,
    },
};
use box2d_rs::shapes::b2_chain_shape::B2chainShape;
use specs::{Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct TerrainCollision {
    terrain_listener: Option<Listener>,
}

impl TerrainCollision {
    pub fn new() -> TerrainCollision {
        TerrainCollision {
            terrain_listener: None,
        }
    }
}

impl<'a> System<'a> for TerrainCollision {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, ChunkIndex>,
        WriteStorage<'a, PhysicsBody>,
        Read<'a, UnsafeBox2D>,
        Write<'a, Terrain>,
    );

    fn run(&mut self, (transform, chunk, mut physics_body, box2d, mut terrain): Self::SystemData) {
        let events = match self.terrain_listener {
            Some(listener) => terrain.consume_changes(listener),
            None => {
                // Initialize
                for (chunk, physics_body) in (&chunk, &mut physics_body).join() {
                    let chunk = match terrain.index_to_chunk(&chunk.index) {
                        Some(chunk) => chunk,
                        None => continue,
                    };
                    // TODO: reduce duplicate code
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
                            let (_, physics_body) = match (&chunk, &mut physics_body)
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
                            // TODO: reduce duplicate code
                        }
                        TerrainUpdate::ChunkRemoved(index) => {}
                        TerrainUpdate::TexelsUpdated(index, changes) => {
                            let (_, physics_body) = match (&chunk, &mut physics_body)
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
                            // TODO: reduce duplicate code
                            let islands = marching_square::calculate_collisions(chunk);
                            let mut shapes: Vec<B2chainShape> = Vec::with_capacity(islands.len());
                            for island in islands {
                                shapes.push(create_segmented_shape(island))
                            }

                            replace_shape(physics_body.body.clone(), vec![], shapes);
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
