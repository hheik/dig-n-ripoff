use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct TextElement {
    text: String,
}

impl TextElement {
    pub fn new() -> TextElement {
        TextElement {
            text: String::new(),
        }
    }

    pub fn from_string(text: &str) -> TextElement {
        TextElement {
            text: text.to_string(),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
