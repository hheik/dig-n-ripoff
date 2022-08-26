use components::*;
use gl::renderer::UnsafeCanvas;
use mst::world_gen;
use resources::{Time, Camera};
use specs::{Builder, DispatcherBuilder, World, WorldExt, shred::FetchMut};
use std::{time::Duration};
use systems::*;
use util::{Vector2, Vector2I};

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

mod components;
mod gl;
mod mst;
mod systems;
mod util;
mod resources;

pub fn main() {
    let lifetime = std::time::SystemTime::now();

    let mut world = World::new();
    world.register::<Transform>();
    world.register::<Chunk>();
    world.register::<RenderTarget>();

    // Init window
    let (_, canvas, mut event_pump): (Sdl, UnsafeCanvas, EventPump) = gl::renderer::init();

    let camera = Camera {
        transform: Transform::new(Vector2 { x: 0.0, y: 0.0 }, 0.0, Vector2 { x: 4.0, y: 4.0 }),
    };

    let time = Time {
        delta_time: Duration::new(0, 0),
        lifetime,
        frame: 0
    };

    world.insert(time);
    world.insert(camera);
    world.insert(canvas);

    let now = std::time::SystemTime::now();
    for y in 0..256 / Chunk::SIZE_Y as i32 {
        for x in 0..256 / Chunk::SIZE_X as i32 {
            let chunk = world_gen::gen_chunk(Vector2I { x, y });
            world
                .create_entity()
                .with(Transform::new(
                    Vector2 {
                        x: (x * Chunk::SIZE_X as i32) as f32,
                        y: (y * Chunk::SIZE_Y as i32) as f32,
                    },
                    0.0,
                    Vector2 { x: 1.0, y: 1.0 },
                ))
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
        .with(CameraControl, "camera_control", &[])
        .with_thread_local(TerrainRender)
        .with_thread_local(Render)
        .build();
    
    'running: loop {
        let now = std::time::SystemTime::now();

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

        let mut time: FetchMut<Time> = world.fetch_mut();
        time.frame = (time.frame + 1) % u64::MAX;
        time.delta_time = match now.elapsed() {
            Ok(elapsed) => elapsed,
            Err(error) => panic!("Delta timer error: {:?}", error),
        }
    }
}
