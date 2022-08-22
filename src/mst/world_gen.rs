use super::{texel::Texel, utils::*};
use crate::{components::Chunk, util::Vector2I};
use sdl2::{
    pixels::{self, Color},
    rect::Rect,
    surface::Surface,
};
use std::collections::HashMap;

const DATA_PATH: &str = "./assets/terrain/complex_terrain.png";

fn read_image(path: &str) -> Surface<'static> {
    match sdl2::image::LoadSurface::from_file(path) {
        Ok(surface) => surface,
        Err(error) => panic!("Error opening image: {:?}", error),
    }
}

fn color_dist(a: &Color, b: &Color) -> u16 {
    a.r.abs_diff(b.r) as u16 + a.g.abs_diff(b.g) as u16 + a.b.abs_diff(b.b) as u16
}

fn map_closest_color(color: Color, map: &HashMap<Color, Texel>) -> Texel {
    let mut closest = map.iter().next().unwrap().0;
    for pair in map.iter() {
        if color_dist(&color, pair.0) < color_dist(&color, closest) {
            closest = pair.0;
        }
    }
    map[closest]
}

pub fn gen_chunk(position_index: Vector2I) -> Chunk {
    let color_map: HashMap<Color, Texel> = [
        (Color::RGB(172, 191, 250), Texel::empty()),
        (Color::RGB(30, 30, 30), Texel::empty()),
        (Color::RGB(0, 0, 0), Texel::empty()),
        (Color::RGB(158, 127, 99), Texel { id: 1 }),
        (Color::RGB(70, 142, 71), Texel { id: 2 }),
    ]
    .iter()
    .cloned()
    .collect();

    let start = chunk_index_to_global(&position_index);
    let size = Vector2I {
        x: Chunk::SIZE_X as i32,
        y: Chunk::SIZE_Y as i32,
    };

    let tex_surface = read_image(DATA_PATH);
    let format = pixels::PixelFormatEnum::RGBA32;
    let mut chunk_surface = Surface::new(size.x as u32, size.y as u32, format).unwrap();
    let image_rect = Rect::new(start.x, start.y, size.x as u32, size.y as u32);

    match tex_surface.blit(
        image_rect,
        &mut chunk_surface,
        Rect::new(0, 0, size.x as u32, size.y as u32),
    ) {
        Ok(_rect) => _rect,
        Err(error) => panic!("Error blitting image: {:?}", error),
    };

    let mut texels = Chunk::new_texel_array();
    chunk_surface.with_lock(|p_data| {
        let mut p_iter = p_data.iter();
        for y in 0..chunk_surface.height() as usize {
            for x in 0..chunk_surface.width() as usize {
                let p_color = Color {
                    r: *p_iter.next().unwrap(),
                    g: *p_iter.next().unwrap(),
                    b: *p_iter.next().unwrap(),
                    a: *p_iter.next().unwrap(),
                };
                texels[y * Chunk::SIZE_X + x] = map_closest_color(p_color, &color_map);
            }
        }
    });

    Chunk {
        texels,
        is_dirty: false,
    }
}
