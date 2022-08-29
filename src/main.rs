use components::*;
use gl::renderer::UnsafeCanvas;
use resources::{Box2D, Camera, Terrain, Time};
use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};
use specs::{shred::FetchMut, DispatcherBuilder, World, WorldExt};
use systems::*;
use util::Vector2;

mod components;
mod gl;
mod mst;
mod resources;
mod systems;
mod util;

pub fn main() {
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

    let time = Time::default();

    let camera = Camera {
        transform: Transform::new(Vector2 { x: 0.0, y: 0.0 }, 0.0, Vector2 { x: 4.0, y: 4.0 }),
    };

    let box2d_world = Box2D::new_unsafe();

    world.insert(terrain);
    world.insert(time);
    world.insert(camera);
    world.insert(canvas);
    world.insert(box2d_world);

    let mut dispatcher = DispatcherBuilder::new()
        .with(TerrainPainter, "terrain_painter", &[])
        .with(TerrainSync::new(), "terrain_sync", &[])
        // .with(CameraControl, "camera_control", &[])
        .with_thread_local(Box2DPhysics::new())
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
        world.maintain();
        {
            let mut terrain: FetchMut<Terrain> = world.fetch_mut();
            for (_, chunk) in terrain.chunk_iter_mut() {
                chunk.is_dirty = false;
            }
        }

        let mut time: FetchMut<Time> = world.fetch_mut();
        time.frame = (time.frame + 1) % u64::MAX;
        time.delta_time = match now.elapsed() {
            Ok(elapsed) => {
                // println!("fps: {}", (1.0 / elapsed.as_secs_f32()).round() as i32);
                elapsed
            }
            Err(error) => panic!("Delta timer error: {:?}", error),
        }
    }
}
