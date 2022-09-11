use std::f32::consts::PI;

use box2d_rs::b2_body::B2bodyType;
use components::{
    flags,
    ui::{self, ElementShadow},
    *,
};
use gl::renderer::{self, UnsafeCanvas};
use resources::{Box2D, Camera, Input, InputState, Terrain, Time};
use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};
use specs::{
    shred::{Fetch, FetchMut},
    Builder, DispatcherBuilder, World, WorldExt,
};

use util::{box2d::create_box, SortingOrder, Vector2, Vector2F};

use crate::{resources::MouseButton, util::Vector2I};

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
    world.register::<PhysicsBody>();
    world.register::<ui::TextElement>();
    world.register::<ui::ElementShadow>();
    world.register::<flags::DebugText>();

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

    let box2d = Box2D::new_unsafe();
    let box2d_world = box2d.world_ptr.clone();

    let pyramid_size = 1;
    let center = 128.0;
    let box_size = 12.0;
    let separation = 1.5;
    for y in 0..pyramid_size {
        for x in 0..y + 1 {
            let c: u8 = (x * 64 % 256) as u8;
            let pos = Vector2F {
                x: center + (x as f32 - y as f32 / 2.0) * box_size * separation,
                y: y as f32 * box_size * separation + 16.0,
            };
            create_box(
                &mut world,
                box2d_world.clone(),
                B2bodyType::B2DynamicBody,
                pos,
                x as f32 * PI / 8.0,
                Vector2F::ONE * box_size,
                (255 - c, 255, c, 255),
            )
        }
    }

    world.insert(terrain);
    world.insert(time);
    world.insert(camera);
    world.insert(canvas);
    world.insert(box2d);
    world.insert(Input::new());

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::TerrainPainter::new(), "terrain_painter", &[])
        .with(systems::CameraControl::new(), "camera_control", &[])
        .with(systems::debug::DebugInfo::new(), "debug_info", &[])
        .with_thread_local(systems::TerrainSync::new())
        .with_thread_local(systems::TerrainCollision::new())
        .with_thread_local(systems::Box2DPhysics::new())
        .with_thread_local(systems::TerrainRender::new())
        .with_thread_local(systems::ui::UIRender::new())
        .with_thread_local(systems::Render)
        .with_thread_local(systems::Box2DVisualizer)
        .build();

    world
        .create_entity()
        .with(Transform::IDENTITY.with_position(Vector2F { x: 4.0, y: 4.0 }))
        .with(RenderTarget::new(
            1,
            1,
            Vector2F::ZERO,
            SortingOrder::Ui as i16,
            true,
        ))
        .with(ui::TextElement::from_string("fps: 0"))
        .with(ui::ElementShadow::new())
        .with(flags::DebugText)
        .build();

    let mut mouse_state;

    'running: loop {
        let now = std::time::SystemTime::now();

        {
            let input: Fetch<Input> = world.fetch();
            mouse_state = input.curr_state().mouse;
        }

        mouse_state.scroll = Vector2I::ZERO;

        // TODO: Move out of main.rs
        // TODO: Fix lingering mousedown bug
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn,
                    clicks: _,
                    x: _,
                    y: _,
                } => {
                    let button = MouseButton::from(mouse_btn);
                    mouse_state.set_button_state(&button, true);
                }
                Event::MouseButtonUp {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn,
                    clicks: _,
                    x: _,
                    y: _,
                } => {
                    let button = MouseButton::from(mouse_btn);
                    mouse_state.set_button_state(&button, false);
                }
                Event::MouseWheel {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x,
                    y,
                    direction: _,
                } => {
                    mouse_state.scroll = Vector2I { x, y };
                }
                Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate: _,
                    x,
                    y,
                    xrel,
                    yrel,
                } => {
                    mouse_state.position = Vector2I { x, y };
                    mouse_state.velocity = Vector2I { x: xrel, y: yrel };
                }
                _ => {}
            }
        }

        {
            let mut input: FetchMut<Input> = world.fetch_mut();
            input.push_state(InputState { mouse: mouse_state });
        }

        {
            let mut canvas: FetchMut<UnsafeCanvas> = world.fetch_mut();
            renderer::begin_draw(&mut canvas);
        }

        dispatcher.dispatch(&world);
        world.maintain();

        {
            let mut canvas: FetchMut<UnsafeCanvas> = world.fetch_mut();
            renderer::finish_draw(&mut canvas);
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
