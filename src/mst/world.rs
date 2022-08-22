use sdl2::surface::Surface;

use super::{
    chunk::{self, Chunk, CHUNK_SIZE_X, CHUNK_SIZE_Y},
    chunk_renderer::ChunkRenderer,
};
use crate::{util::Vector2I, SURFACE_FORMAT};

pub struct World {
    pub chunks: Box<Vec<ChunkRenderer>>,
}

impl World {
    pub fn new() -> World {
        World {
            chunks: Box::new(Vec::new()),
        }
    }

    pub fn load_chunk(&mut self, chunk: Chunk) {
        let surface: Surface<'static> =
            match Surface::new(CHUNK_SIZE_X as u32, CHUNK_SIZE_Y as u32, SURFACE_FORMAT) {
                Ok(surface) => surface,
                Err(error) => panic!("Failed to create chunk SDL surface: {error:?}"),
            };
        // let palette: Palette = match Palette::with_colors(&[
        //     Color::RGBA(0, 0, 0, 0),
        //     Color::RGBA(158, 127, 99, 255),
        //     Color::RGBA(70, 142, 71, 255),
        // ]) {
        //     Ok(palette) => palette,
        //     Err(error) => panic!("Failed to create palette: {error:?}"),
        // };
        // match surface.set_palette(&palette) {
        //     Ok(surface) => surface,
        //     Err(error) => panic!("Failed setting palette to surface: {error:?}"),
        // };
        self.chunks.push(ChunkRenderer { chunk, surface });
    }

    pub fn global_to_chunk_index(pos: &Vector2I) -> Vector2I {
        Vector2I {
            x: Self::wrapping_quotient(pos.x, chunk::CHUNK_SIZE_X as i32),
            y: Self::wrapping_quotient(pos.y, chunk::CHUNK_SIZE_Y as i32),
        }
    }

    pub fn chunk_index_to_global(ci: &Vector2I) -> Vector2I {
        Vector2I {
            x: ci.x * chunk::CHUNK_SIZE_X as i32,
            y: ci.y * chunk::CHUNK_SIZE_Y as i32,
        }
    }

    /** Calculate quotient, but take into account negative values so that they continue the cycle seamlessly.
        e.g. (0, 4) -> 0, (-4, 4) -> -1, (-5, 4) -> -2
    */
    fn wrapping_quotient(dividend: i32, divisor: i32) -> i32 {
        let res = (if dividend < 0 { dividend + 1 } else { dividend }) / divisor;
        if dividend < 0 {
            res - 1
        } else {
            res
        }
    }
}
