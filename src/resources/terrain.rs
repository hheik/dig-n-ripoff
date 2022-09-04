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
    util::Vector2I,
};

#[derive(Default)]
pub struct Terrain {
    chunk_map: HashMap<Vector2I, Chunk>,
    on_texel_update: ,
}

impl Terrain {
    pub fn new() -> Terrain {
        let mut terrain = Terrain {
            chunk_map: HashMap::new(),
            on_texel_update: Vec::new(),
        };
        for (index, chunk) in gen_from_image().drain() {
            terrain.add_chunk(index, chunk);
        }
        terrain
    }

    fn on_chunk_update(&self, index: Vector2I, id: TexelID) {}

    pub fn add_chunk(&mut self, index: Vector2I, mut chunk: Chunk) {
        chunk.add_listener(|index, id| {
            for update in self.on_texel_update.clone() {
                println!("something changed: {} -> {}", index, id);
            }
        });
        self.chunk_map.insert(index, chunk);
    }

    pub fn remove_chunk(&mut self, index: Vector2I) {
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
            Some(chunk) => Some(chunk.get_texel(&global_to_local(global))),
            None => None,
        }
    }

    pub fn global_to_texel_mut(&mut self, global: &Vector2I) -> Option<Texel> {
        match self.global_to_chunk(global) {
            Some(chunk) => Some(chunk.get_texel(&global_to_local(global))),
            None => None,
        }
    }

    pub fn set_texel(&mut self, global: &Vector2I, id: TexelID) {
        match self.global_to_chunk_mut(global) {
            Some(chunk) => chunk.set_texel(&global_to_local(global), id),
            None => {}
        }
    }

    // pub fn add_texel_listener(&mut self, cb: TexelUpdate) {
    //     self.on_texel_update.push(cb);
    // }
}
