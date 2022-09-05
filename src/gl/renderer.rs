use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::{Point, Rect},
    render::Canvas,
    surface::Surface,
    video::Window,
    EventPump, Sdl,
};

pub const INIT_WINDOW_SIZE: (u32, u32) = (1024, 1024);
pub const SURFACE_FORMAT: PixelFormatEnum = PixelFormatEnum::RGBA32;
pub const SURFACE_FORMAT_BPP: usize = 4;

use unsafe_send_sync::UnsafeSendSync;

use crate::util::Vector2I;

pub type UnsafeSurface<'a> = UnsafeSendSync<Surface<'a>>;
pub type UnsafeCanvas = UnsafeSendSync<Canvas<Window>>;

pub fn init() -> (Sdl, UnsafeCanvas, EventPump) {
    let sdl_context = match sdl2::init() {
        Ok(context) => context,
        Err(error) => panic!("Failed to init sdl context: {error:?}"),
    };
    let video_subsystem = match sdl_context.video() {
        Ok(video) => video,
        Err(error) => panic!("Failed to get sdl video subsystem: {error:?}"),
    };

    let window = match video_subsystem
        .window("rust-sdl2 demo", INIT_WINDOW_SIZE.0, INIT_WINDOW_SIZE.1)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(error) => panic!("Failed to create window: {error:?}"),
    };

    let canvas = match window.into_canvas().build() {
        Ok(canvas) => UnsafeCanvas::new(canvas),
        Err(error) => panic!("Failed to create window canvas: {error:?}"),
    };

    let event_pump = match sdl_context.event_pump() {
        Ok(event_pump) => event_pump,
        Err(error) => panic!("Failed to create event pump: {error:?}"),
    };

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

pub fn draw_surface_rotated(
    canvas: &mut UnsafeCanvas,
    surface: &UnsafeSurface,
    src: Rect,
    dst: Rect,
    angle: f64,
    pivot: Point,
    flip_h: bool,
    flip_v: bool,
) {
    let texture_creator = canvas.texture_creator();
    let texture = match surface.as_texture(&texture_creator) {
        Ok(texture) => texture,
        Err(error) => panic!("Failed to create texture from surface: {error:?}"),
    };
    match canvas.copy_ex(
        &texture,
        src,
        dst,
        angle.to_degrees(),
        pivot,
        flip_h,
        flip_v,
    ) {
        Ok(_) => {}
        Err(error) => panic!("Failed to draw surface to canvas: {error:?}"),
    };
}

pub fn draw_line_loop(
    canvas: &mut UnsafeCanvas,
    points: Vec<Vector2I>,
    color: Option<(u8, u8, u8, u8)>,
) {
    let color = match color {
        Some(color) => Color::RGBA(color.0, color.1, color.2, color.3),
        None => Color::RGBA(0, 255, 255, 128),
    };
    canvas.set_draw_color(color);
    let mut p: Vec<Point> = Vec::new();
    for point in points {
        p.push(Point::new(point.x, point.y));
    }
    if p.first().is_some() && p.last().is_some() && p.first() != p.last() {
        p.push(*p.first().unwrap());
    }
    match canvas.draw_lines(&p[..]) {
        Ok(_) => {}
        Err(error) => panic!("Failed to draw points: {error:?}"),
    };
}
