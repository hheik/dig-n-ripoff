use std::path::Path;
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use lazy_static::lazy_static;
use sdl2::ttf::{Font, Sdl2TtfContext};
use unsafe_send_sync::UnsafeSendSync;

use crate::gl::renderer::UnsafeFont;

const FONT_DIR: &str = "./assets/fonts/";
const DEFAULT_FONT_NAME: &str = "venice_classic";
const DEFAULT_FONT_SIZE: u16 = 19;

lazy_static! {
    static ref FONT_CONTEXT: Sdl2TtfContext =
        sdl2::ttf::init().expect("Could not initialize SDL2 ttf context");
    pub static ref DEFAULT_FONT: Arc<Mutex<UnsafeFont>> = Arc::new(Mutex::new(
        UnsafeSendSync::new(load_font(DEFAULT_FONT_NAME, DEFAULT_FONT_SIZE))
    ));
}

pub fn load_font(name: &str, size: u16) -> Font<'static, 'static> {
    FONT_CONTEXT
        .load_font(Path::new(FONT_DIR).join(name.to_string() + ".ttf"), size)
        .expect(&("Failed to load font ".to_string() + name))
}
