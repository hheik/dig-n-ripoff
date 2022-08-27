use components::*;
use gl::renderer::UnsafeCanvas;
use resources::{Camera, Terrain, Time};
use specs::{shred::FetchMut, DispatcherBuilder, World, WorldExt};
use std::time::Duration;
use systems::*;
use util::Vector2;

use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

mod components;
mod gl;
mod mst;
mod resources;
mod systems;
mod util;

pub fn main() {
    let lifetime = std::time::SystemTime::now();

    let mut world = World::new();
    world.register::<Transform>();
    world.register::<ChunkIndex>();
    world.register::<RenderTarget>();

    // Init window
    let (_, canvas, mut event_pump): (Sdl, UnsafeCanvas, EventPump) = gl::renderer::init();

    let now = std::time::SystemTime::now();
    let terrain = Terrain::new();
    match now.elapsed() {
        Ok(elapsed) => println!("Creating chunks from image took {}ms", elapsed.as_millis()),
        Err(error) => println!("Timer error: {:?}", error),
    };

    let time = Time {
        delta_time: Duration::new(0, 0),
        lifetime,
        frame: 0,
    };

    let camera = Camera {
        transform: Transform::new(Vector2 { x: 0.0, y: 0.0 }, 0.0, Vector2 { x: 4.0, y: 4.0 }),
    };

    world.insert(terrain);
    world.insert(canvas);
    world.insert(time);
    world.insert(camera);

    let mut dispatcher = DispatcherBuilder::new()
        // .with(CameraControl, "camera_control", &[])
        .with(TerrainPainter, "terrain_painter", &[])
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
