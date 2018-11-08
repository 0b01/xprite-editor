use xprite::prelude::*;

pub struct State {
    pub xpr: Xprite,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            xpr
        }
    }
}