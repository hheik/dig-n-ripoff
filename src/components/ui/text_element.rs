use std::sync::{Arc, Mutex};

use sdl2::{pixels::Color, ttf::Font};
use specs::{Component, VecStorage};
use unsafe_send_sync::UnsafeSendSync;

use crate::util::font::DEFAULT_FONT;

#[derive(Component)]
#[storage(VecStorage)]
pub struct TextElement {
    font: Arc<Mutex<UnsafeSendSync<Font<'static, 'static>>>>,
    text: String,
    color: Color,
}

impl TextElement {
    const DEFAULT_COLOR: Color = Color {
        r: 228,
        g: 225,
        b: 216,
        a: 255,
    };

    pub fn new() -> TextElement {
        TextElement {
            font: (*DEFAULT_FONT).clone(),
            text: String::new(),
            color: Self::DEFAULT_COLOR,
        }
    }

    pub fn from_string(text: &str) -> TextElement {
        let mut element = TextElement::new();
        element.text = text.to_string();
        element
    }

    pub fn get_font(&self) -> &Arc<Mutex<UnsafeSendSync<Font<'static, 'static>>>> {
        &self.font
    }

    pub fn set_font(&mut self, font: Arc<Mutex<UnsafeSendSync<Font<'static, 'static>>>>) {
        self.font = font;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
