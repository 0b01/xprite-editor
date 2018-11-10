#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InputItem {
    Ctrl,
    Shift,
    Z,
    Y,
    Left,
    Right,
}

impl InputItem {
    pub fn as_str(&self) -> &str {
        match &self {
            InputItem::Ctrl => "ctrl",
            InputItem::Shift => "shift",
            InputItem::Z => "z",
            InputItem::Y => "y",
            _ => unimplemented!(),
        }
    }
}


#[derive(Debug)]
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
    pub y: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            ctrl: false,
            shift: false,
            z: false,
            y: false,
        }
    }
}


impl InputState {
    pub fn debounce(&mut self, item: InputItem, new_state: bool) -> bool {
        macro_rules! debounce {
            ($a:ident) => {
                if self.$a != new_state {
                    self.$a = !self.$a;
                    true
                } else {
                    false
                }
            };
        }
        match item {
            InputItem::Ctrl => debounce!(ctrl),
            InputItem::Shift => debounce!(shift),
            InputItem::Z => debounce!(z),
            InputItem::Y => debounce!(y),
            InputItem::Left => debounce!(left),
            InputItem::Right => debounce!(right),
        }
    }
}
