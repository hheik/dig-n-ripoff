use mst::{
    chunk::{CHUNK_SIZE_X, CHUNK_SIZE_Y},
    world::World,
    world_gen,
};
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use sdl2::{keyboard::Keycode, pixels::PixelFormatEnum};
use std::time::Duration;
use util::Vector2I;

mod mst;
mod util;

pub const SURFACE_FORMAT: PixelFormatEnum = PixelFormatEnum::RGBA32;
pub const SURFACE_FORMAT_BPP: usize = 4;

// pub const SURFACE_FORMAT: PixelFormatEnum = PixelFormatEnum::Index8;
// pub const SURFACE_FORMAT_BPP: usize = 1;

pub fn main() {
    let mut world = World::new();
    let now = std::time::SystemTime::now();
    for y in 0..256 / CHUNK_SIZE_Y as i32 {
        for x in 0..256 / CHUNK_SIZE_X as i32 {
            let chunk = world_gen::gen_chunk(Vector2I { x, y });
            world.load_chunk(chunk);
        }
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Creating chunks from image took {}ms", elapsed.as_millis()),
        Err(error) => println!("Timer error: {:?}", error),
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 1024, 1024)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // Update chunk surfaces
        for renderer in world.chunks.iter_mut() {
            renderer.draw_to_surface(SURFACE_FORMAT_BPP);
        }

        // Copy chunk surfaces to canvas
        for renderer in world.chunks.iter() {
            let scale = 4;
            let pos = (World::chunk_index_to_global(&renderer.chunk.position_index)) * scale;
            let size = Vector2I {
                x: CHUNK_SIZE_X as i32,
                y: CHUNK_SIZE_Y as i32,
            } * scale;
            let dst_rect = Rect::new(pos.x, pos.y, size.x as u32, size.y as u32);

            match canvas.copy(
                match &renderer.surface.as_texture(&texture_creator) {
                    Ok(tex) => tex,
                    Err(error) => panic!("Error applying surface as texture: {error:?}"),
                },
                renderer.surface.rect(),
                dst_rect,
            ) {
                Ok(copy) => copy,
                Err(error) => panic!("Error copying chunk surface to canvas: {error:?}"),
            };
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
