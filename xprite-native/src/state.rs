use xprite::prelude::*;

pub struct State {
    pub xpr: Xprite,
    pub show_settings: bool,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            xpr,
            show_settings: false,
        }
    }

}