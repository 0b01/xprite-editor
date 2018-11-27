use xprite::prelude::*;
use crate::prelude::*;

pub struct State {
    pub xpr: Xprite,
    pub show_settings: bool,
    pub hotkeys: Hotkey,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            xpr,
            show_settings: false,
            hotkeys: Hotkey::new(),
        }
    }

}