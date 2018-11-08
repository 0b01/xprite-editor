#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InputItem {
    Ctrl,
    Shift,
    Z,
    Left,
    Right,
}

pub enum InputEvent {
    MouseMove {
        x: f32,
        y: f32,
    },
    MouseDown {
        x: f32,
        y: f32,
        button: InputItem,
    },
    MouseUp {
        x: f32,
        y: f32,
    },
    KeyUp {
        key: InputItem,
    },
    KeyDown {
        key: InputItem,
    }
}


pub struct InputState {
    pub left: bool,
    pub right: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub z: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            ctrl: false,
            shift: false,
            z: false,
        }
    }
}


impl InputState {
    pub fn debounce(&mut self, item: InputItem, new_state: bool) -> bool {
        macro_rules! debounce {
            ($a:ident) => {
                if self.$a != new_state {
                    self.$a = new_state;
                    true
                } else {
                    false
                }
            };
        }
        match item {
            Ctrl => debounce!(ctrl),
            Shift => debounce!(shift),
            Z => debounce!(z),
            Left => debounce!(left),
            Right => debounce!(right),
        }
    }
}
