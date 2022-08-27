use specs::{
    join::JoinIter,
    shred::{ResourceId, SystemData, World},
    Join, ReadStorage, WriteStorage,
};

use crate::{
    components::{Chunk, Transform},
    mst::texel::Texel,
};

use super::{math::Mathf32::wrapping_quotient, Vector2I};

#[derive(SystemData)]
pub struct ChunkReadStorage<'a> {
    pub chunk: ReadStorage<'a, Chunk>,
    pub transform: ReadStorage<'a, Transform>,
}
impl<'a> ChunkReadStorage<'a> {
    pub fn join(&self) -> JoinIter<(&ReadStorage<'a, Chunk>, &ReadStorage<'a, Transform>)> {
        (&self.chunk, &self.transform).join()
    }
}

#[derive(SystemData)]
pub struct ChunkWriteStorage<'a> {
    pub chunk: WriteStorage<'a, Chunk>,
    pub transform: ReadStorage<'a, Transform>,
}
impl<'a> ChunkWriteStorage<'a> {
    pub fn join(
        &mut self,
    ) -> JoinIter<(&mut WriteStorage<'a, Chunk>, &ReadStorage<'a, Transform>)> {
        (&mut self.chunk, &self.transform).join()
    }

    fn global_to_local(position: Vector2I) -> Vector2I {
        Vector2I {
            x: position.x % Chunk::SIZE_X as i32,
            y: position.y % Chunk::SIZE_Y as i32,
        }
    }

    pub fn global_to_chunk_index(position: Vector2I) -> Vector2I {
        let centered = position
            + Vector2I {
                x: Chunk::SIZE_X as i32,
                y: Chunk::SIZE_Y as i32,
            } / 2;
        Vector2I {
            x: wrapping_quotient(centered.x, Chunk::SIZE_X as i32),
            y: wrapping_quotient(centered.y, Chunk::SIZE_Y as i32),
        }
    }

    fn global_to_chunk(&mut self, position: Vector2I) -> Option<&mut Chunk> {
        let mut result = None;
        for (chunk, transform) in self.join() {
            if Self::global_to_chunk_index(position)
                == Self::global_to_chunk_index(transform.get_position().rounded())
            {
                result = Some(chunk);
                break;
            }
        }
        result
    }

    pub fn get_texel(&mut self, position: Vector2I) -> Option<Texel> {
        match self.global_to_chunk(position) {
            Some(chunk) => Some(chunk.get_texel(position)),
            None => None,
        }
    }

    pub fn set_texel(&mut self, position: Vector2I, value: Texel) {
        match self.global_to_chunk(position) {
            Some(chunk) => chunk.set_texel(position, value),
            None => {}
        }
    }
}
