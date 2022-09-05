use std::collections::{
    hash_map::{Iter, IterMut},
    HashMap,
};

use crate::{
    mst::{
        chunk::{Chunk, TexelUpdate},
        texel::{Texel, TexelID},
        utils::{global_to_index, global_to_local},
        world_gen::gen_from_image,
    },
    util::{ChangeBuffer, Listener, Vector2I},
};

#[derive(Clone)]
pub enum TerrainUpdate {
    None,
    ChunkAdded(Vector2I),
    ChunkRemoved(Vector2I),
    TexelsUpdated(Vector2I, Vec<TexelUpdate>),
}

impl Default for TerrainUpdate {
    fn default() -> Self {
        TerrainUpdate::None
    }
}

#[derive(Default)]
pub struct Terrain {
    chunk_map: HashMap<Vector2I, Chunk>,
    change_buffer: ChangeBuffer<TerrainUpdate>,
    /// Map a single listener to per-chunk listeners
    chunk_listener_map: HashMap<Listener, HashMap<Vector2I, Listener>>,
}

impl Terrain {
    pub fn new() -> Terrain {
        let mut terrain = Terrain {
            chunk_map: HashMap::new(),
            chunk_listener_map: HashMap::new(),
            change_buffer: ChangeBuffer::new(),
        };
        for (index, chunk) in gen_from_image().drain() {
            terrain.add_chunk(index, chunk);
        }
        terrain
    }

    pub fn add_chunk(&mut self, index: Vector2I, chunk: Chunk) {
        self.chunk_map.insert(index, chunk);
        self.change_buffer
            .push_event(TerrainUpdate::ChunkAdded(index));
    }

    pub fn remove_chunk(&mut self, index: Vector2I) {
        self.change_buffer
            .push_event(TerrainUpdate::ChunkRemoved(index));
        self.chunk_map.remove(&index);
    }

    pub fn chunk_iter(&self) -> Iter<Vector2I, Chunk> {
        self.chunk_map.iter()
    }

    pub fn chunk_iter_mut(&mut self) -> IterMut<Vector2I, Chunk> {
        self.chunk_map.iter_mut()
    }

    pub fn index_to_chunk(&self, index: &Vector2I) -> Option<&Chunk> {
        self.chunk_map.get(index)
    }

    pub fn index_to_chunk_mut(&mut self, index: &Vector2I) -> Option<&mut Chunk> {
        self.chunk_map.get_mut(index)
    }

    pub fn global_to_chunk(&self, global: &Vector2I) -> Option<&Chunk> {
        self.index_to_chunk(&global_to_index(global))
    }

    pub fn global_to_chunk_mut(&mut self, global: &Vector2I) -> Option<&mut Chunk> {
        self.index_to_chunk_mut(&global_to_index(global))
    }

    pub fn global_to_texel(&self, global: &Vector2I) -> Option<Texel> {
        match self.global_to_chunk(global) {
            Some(chunk) => chunk.get_texel(&global_to_local(global)),
            None => None,
        }
    }

    pub fn global_to_texel_mut(&mut self, global: &Vector2I) -> Option<Texel> {
        match self.global_to_chunk(global) {
            Some(chunk) => chunk.get_texel(&global_to_local(global)),
            None => None,
        }
    }

    pub fn set_texel(&mut self, global: &Vector2I, id: TexelID) {
        match self.global_to_chunk_mut(global) {
            Some(chunk) => chunk.set_texel(&global_to_local(global), id),
            None => {}
        }
    }

    pub fn get_listener(&mut self) -> Listener {
        let listener = self.change_buffer.get_listener();

        let mut chunk_listeners: HashMap<Vector2I, Listener> = HashMap::new();
        for (index, chunk) in self.chunk_map.iter_mut() {
            chunk_listeners.insert(*index, chunk.change_buffer.get_listener());
        }
        self.chunk_listener_map.insert(listener, chunk_listeners);

        listener
    }

    pub fn consume_changes(&mut self, listener: Listener) -> Option<Vec<TerrainUpdate>> {
        match self.chunk_listener_map.get(&listener) {
            Some(listeners) => {
                for (index, listener) in listeners {
                    match self.chunk_map.get_mut(index) {
                        Some(chunk) => match chunk.change_buffer.consume_listener(*listener) {
                            Some(changes) => {
                                if changes.len() > 0 {
                                    self.change_buffer
                                        .push_event(TerrainUpdate::TexelsUpdated(*index, changes));
                                }
                            }
                            None => (),
                        },
                        None => (),
                    }
                }
                self.chunk_listener_map.remove(&listener);
            }
            None => (),
        };

        self.change_buffer.consume_listener(listener)
    }
}
