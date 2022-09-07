use super::{
    chunk::Chunk,
    texel::{Texel, TexelID},
    utils::{global_to_index, global_to_local},
};
use crate::util::Vector2I;
use sdl2::{pixels::Color, surface::Surface};
use std::collections::HashMap;

// const DATA_PATH: &str = "./assets/terrain/noise.png";
const DATA_PATH: &str = "./assets/terrain/complex_terrain.png";
// const DATA_PATH: &str = "./assets/terrain/terrain.png";
// const DATA_PATH: &str = "./assets/terrain/simple_terrain.png";
// const DATA_PATH: &str = "./assets/terrain/chunk_0.png";
// const DATA_PATH: &str = "./assets/terrain/chunk_1.png";
// const DATA_PATH: &str = "./assets/terrain/solid_4x4.png";
// const DATA_PATH: &str = "./assets/terrain/vertical.png";

fn read_image(path: &str) -> Surface<'static> {
    match sdl2::image::LoadSurface::from_file(path) {
        Ok(surface) => surface,
        Err(error) => panic!("Error opening image: {:?}", error),
    }
}

fn color_dist(a: &Color, b: &Color) -> u16 {
    a.r.abs_diff(b.r) as u16 + a.g.abs_diff(b.g) as u16 + a.b.abs_diff(b.b) as u16
}

fn map_closest_color(color: Color, map: &HashMap<Color, TexelID>) -> TexelID {
    let mut closest = map.iter().next().unwrap().0;
    for pair in map.iter() {
        if color_dist(&color, pair.0) < color_dist(&color, closest) {
            closest = pair.0;
        }
    }
    map[closest]
}

pub fn gen_from_image() -> HashMap<Vector2I, Chunk> {
    let mut chunk_map: HashMap<Vector2I, Chunk> = HashMap::new();

    let color_map: HashMap<Color, TexelID> = [
        (Color::RGB(172, 191, 250), Texel::EMPTY),
        (Color::RGB(30, 30, 30), Texel::EMPTY),
        (Color::RGB(0, 0, 0), Texel::EMPTY),
        (Color::RGB(158, 127, 99), 1),
        (Color::RGB(70, 142, 71), 2),
    ]
    .iter()
    .cloned()
    .collect();

    let tex_surface = read_image(DATA_PATH);

    tex_surface.with_lock(|p_data| {
        let mut p_iter = p_data.iter();
        for y in 0..tex_surface.height() as i32 {
            for x in 0..tex_surface.width() as i32 {
                let p_color = Color {
                    r: *p_iter.next().unwrap(),
                    g: *p_iter.next().unwrap(),
                    b: *p_iter.next().unwrap(),
                    a: *p_iter.next().unwrap(),
                };
                let global = Vector2I { x, y };
                let local = global_to_local(&global);
                let index = global_to_index(&global);
                if !chunk_map.contains_key(&index) {
                    chunk_map.insert(index, Chunk::new());
                }
                match chunk_map.get_mut(&index) {
                    Some(value) => value.set_texel(&local, map_closest_color(p_color, &color_map)),
                    None => (),
                };
            }
        }
    });

    chunk_map
}
