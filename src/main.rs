use components::*;
use mst::world_gen;
use specs::{Builder, RunNow, World, WorldExt};
use std::time::Duration;
use systems::*;
use util::{Vector2, Vector2I};

use sdl2::{
    event::Event,
    keyboard::Keycode,
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump, Sdl,
};

mod components;
mod gl;
mod mst;
mod systems;
mod util;

pub fn main() {
    let mut world = World::new();
    world.register::<Transform>();
    world.register::<Chunk>();

    let now = std::time::SystemTime::now();
    for y in 0..256 / Chunk::SIZE_Y as i32 {
        for x in 0..256 / Chunk::SIZE_X as i32 {
            let chunk = world_gen::gen_chunk(Vector2I { x, y });
            world
                .create_entity()
                .with(Transform::new(Vector2 {
                    x: x as f32,
                    y: y as f32,
                }))
                .with(chunk)
                .build();
        }
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Creating chunks from image took {}ms", elapsed.as_millis()),
        Err(error) => println!("Timer error: {:?}", error),
    }

    let mut terrain_render = TerrainRender;
    terrain_render.run_now(&world);
    world.maintain();

    // Init window
    let (sdl_context, mut canvas, mut event_pump, texture_creator): (
        Sdl,
        Canvas<Window>,
        EventPump,
        TextureCreator<WindowContext>,
    ) = gl::renderer::init();

    let mut frame_counter: u64 = 0;
    'running: loop {
        frame_counter = (frame_counter + 1) % u64::MAX;

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

        gl::renderer::draw(&mut canvas, &texture_creator);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
