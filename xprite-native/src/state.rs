use xprite::prelude::*;

pub struct State {
    pub xpr: Xprite,
    pub last_mouse_pos: (f32, f32),
    pub is_left_mouse_down: bool,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            xpr,
            last_mouse_pos: (0., 0.),
            is_left_mouse_down: false,
        }
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.last_mouse_pos.0 = x;
        self.last_mouse_pos.1 = y;
    }
}