use crate::util::Vector2I;

#[derive(Default, Clone, Copy)]
pub struct MouseState {
    pub buttons: u8,
    pub position: Vector2I,
    pub velocity: Vector2I,
}

impl MouseState {
    pub fn get_button_state(&self, button: &MouseButton) -> bool {
        let index = *button as u8;
        if index == 0 {
            return false
        }
        self.buttons & (1 << (index - 1)) != 0
    }

    pub fn set_button_state(&mut self, button: &MouseButton, state: bool) {
        let index = *button as u8;
        if index == 0 || self.get_button_state(button) == state {
            return
        }
        self.buttons ^= 1 << (index - 1);
    }
}

#[derive(Clone, Copy)]
pub enum MouseButton {
    None = 0,
    Left = 1,
    Middle = 2,
    Right = 3,
    X1 = 4,
    X2 = 5,
}

impl From<sdl2::mouse::MouseButton> for MouseButton {
    fn from(button: sdl2::mouse::MouseButton) -> Self {
        match button {
            sdl2::mouse::MouseButton::Left => MouseButton::Left,
            sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
            sdl2::mouse::MouseButton::Right => MouseButton::Right,
            sdl2::mouse::MouseButton::X1 => MouseButton::X1,
            sdl2::mouse::MouseButton::X2 => MouseButton::X2,
            _ => MouseButton::None,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct InputState {
    pub mouse: MouseState
}

#[derive(Default)]
pub struct Input {
    prev_state: InputState,
    curr_state: InputState,
}

impl Input {
    pub fn new() -> Input {
        Input {
            prev_state: InputState::default(),
            curr_state: InputState::default(),
        }
    }

    pub fn mouse_held(&self, button: MouseButton) -> bool {
        self.curr_state.mouse.get_button_state(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        !self.prev_state.mouse.get_button_state(&button) && self.curr_state.mouse.get_button_state(&button)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.prev_state.mouse.get_button_state(&button) && !self.curr_state.mouse.get_button_state(&button)
    }

    pub fn get_mouse_position(&self) -> Vector2I {
        self.curr_state.mouse.position
    }

    pub fn get_mouse_velocity(&self) -> Vector2I {
        self.curr_state.mouse.velocity
    }

    pub fn push_state(&mut self, state: InputState) {
        self.prev_state = self.curr_state;
        self.curr_state = state;
    }

    pub fn prev_state(&self) -> &InputState {
        &self.prev_state
    }

    pub fn curr_state(&self) -> &InputState {
        &self.prev_state
    }
}
