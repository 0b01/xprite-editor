use xprite::prelude::*;
use crate::prelude::*;

pub struct State {
    pub xpr: Xprite,
    pub show_settings: bool,
    pub hotkeys: HotkeyController,
    pub inputs: InputState,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            xpr,
            show_settings: false,
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
        }
    }

}