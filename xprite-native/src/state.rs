use xprite::prelude::*;

pub struct State {
    pub show_grid: bool,
    pub xpr: Xprite,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            show_grid: false,
            xpr
        }
    }
}