use components::*;
use gl::{camera::Camera, renderer::UnsafeCanvas};
use mst::world_gen;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use std::time::Duration;
use systems::*;
use util::{Vector2, Vector2I};

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

mod components;
mod gl;
mod mst;
mod systems;
mod util;

pub fn main() {
    // Init window
    let (_, canvas, mut event_pump): (Sdl, UnsafeCanvas, EventPump) = gl::renderer::init();

    let camera = Camera {
        position: Vector2 { x: 0.0, y: 0.0 },
        scale: 4.0,
    };

    let mut world = World::new();
    world.register::<Transform>();
    world.register::<Chunk>();
    world.register::<RenderTarget>();

    world.insert(camera);
    world.insert(canvas);

    let now = std::time::SystemTime::now();
    for y in 0..256 / Chunk::SIZE_Y as i32 {
        for x in 0..256 / Chunk::SIZE_X as i32 {
            let chunk = world_gen::gen_chunk(Vector2I { x, y });
            world
                .create_entity()
                .with(Transform::new(Vector2 {
                    x: (x * Chunk::SIZE_X as i32) as f32,
                    y: (y * Chunk::SIZE_Y as i32) as f32,
                }))
                .with(chunk)
                .with(RenderTarget::new(Chunk::SIZE_X, Chunk::SIZE_Y))
                .build();
        }
    }
    match now.elapsed() {
        Ok(elapsed) => println!("Creating chunks from image took {}ms", elapsed.as_millis()),
        Err(error) => println!("Timer error: {:?}", error),
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(TerrainRender)
        .with_thread_local(Render)
        .build();

    let mut frame_counter: u64 = 0;
    'running: loop {
        frame_counter = (frame_counter + 1) % u64::MAX;
        println!("start of frame: {frame_counter}");

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

        dispatcher.dispatch(&world);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
