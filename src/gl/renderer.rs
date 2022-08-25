use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::Canvas,
    surface::Surface,
    video::Window,
    EventPump, Sdl,
};

pub const INIT_WINDOW_SIZE: (u32, u32) = (1024, 1024);
pub const SURFACE_FORMAT: PixelFormatEnum = PixelFormatEnum::RGBA32;
pub const SURFACE_FORMAT_BPP: usize = 4;

use unsafe_send_sync::UnsafeSendSync;

pub type UnsafeSurface<'a> = UnsafeSendSync<Surface<'a>>;
pub type UnsafeCanvas = UnsafeSendSync<Canvas<Window>>;

pub fn init() -> (Sdl, UnsafeCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = UnsafeCanvas::new(window.into_canvas().build().unwrap());
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
        Ok(_) => {}
        Err(error) => panic!("Failed to draw surface to canvas: {error:?}"),
    };
}
