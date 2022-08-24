use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::{Canvas},
    surface::{Surface},
    video::{Window},
    EventPump, Sdl, rect::Rect,
};

pub const INIT_WINDOW_SIZE: (u32, u32) = (1024, 1024);
pub const SURFACE_FORMAT: PixelFormatEnum = PixelFormatEnum::RGBA32;
pub const SURFACE_FORMAT_BPP: usize = 4;

use unsafe_send_sync::UnsafeSendSync;

pub type UnsafeSurface<'a> = UnsafeSendSync<Surface<'a>>;
pub type UnsafeCanvas = UnsafeSendSync<Canvas<Window>>;

pub fn init() -> (
    Sdl,
    UnsafeCanvas,
    EventPump,
) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = UnsafeCanvas::new(window.into_canvas().build().unwrap());
    // let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    (sdl_context, canvas, event_pump)
}

pub fn begin_draw(canvas: &mut UnsafeCanvas) {
    canvas.set_draw_color(Color::RGB(30, 30, 30));
    canvas.clear();
}

pub fn finish_draw(canvas: &mut UnsafeCanvas) {
    canvas.present();
}

pub fn draw_surface(canvas: &mut UnsafeCanvas, surface: &UnsafeSurface, src: Rect, dst: Rect) {
    let texture_creator = canvas.texture_creator();
    let texture = match surface.as_texture(&texture_creator) {
        Ok(texture) => texture,
        Err(error) => panic!("Failed to create texture from surface: {error:?}"),
    };
    match canvas.copy(&texture, src, dst) {
        Ok(_) => {},
        Err(error) => panic!("Failed to draw surface to canvas: {error:?}"),
    };
}

// pub fn draw(canvas: &mut UnsafeCanvas) {
//     // Update chunk surfaces
//     for renderer in world.chunks.iter_mut() {
//         renderer.draw_to_surface(SURFACE_FORMAT_BPP);
//     }

//     // Copy chunk surfaces to canvas
//     for renderer in world.chunks.iter() {
//         let scale = 4;
//         let pos = (World::chunk_index_to_global(&renderer.chunk.position_index)) * scale;
//         let size = Vector2I {
//             x: SIZE_X as i32,
//             y: SIZE_Y as i32,
//         } * scale;
//         let dst_rect = Rect::new(pos.x, pos.y, size.x as u32, size.y as u32);

//         match canvas.copy(
//             match &renderer.surface.as_texture(&texture_creator) {
//                 Ok(tex) => tex,
//                 Err(error) => panic!("Error applying surface as texture: {error:?}"),
//             },
//             renderer.surface.rect(),
//             dst_rect,
//         ) {
//             Ok(copy) => copy,
//             Err(error) => panic!("Error copying chunk surface to canvas: {error:?}"),
//         };
//     }
// }

// pub fn draw_to_surface(chunk: &Chunk, surface: &mut Surface, bytes_per_pixel: usize) {
//     let color_map: HashMap<TexelID, Color> = [
//         (0, Color::RGBA(0, 0, 0, 0)),
//         (1, Color::RGBA(158, 127, 99, 255)),
//         (2, Color::RGBA(70, 142, 71, 255)),
//     ]
//     .iter()
//     .cloned()
//     .collect();

//     surface.with_lock_mut(|p_data| {
//         if p_data.len() != chunk.texels.len() * bytes_per_pixel {
//             panic!("Surface pixel count is not aligned with texel count");
//         }

//         // TODO: This doesn't care about bytes_per_pixel
//         for xy in 0..chunk.texels.len() {
//             let i = xy * bytes_per_pixel;
//             let c = color_map[&chunk.texels[xy].id];
//             p_data[i + 0] = c.r;
//             p_data[i + 1] = c.g;
//             p_data[i + 2] = c.b;
//             p_data[i + 3] = c.a;
//             // p_data[i] = self.chunk.texels[xy].id;
//         }
//     })
// }
