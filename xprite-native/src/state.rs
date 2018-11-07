use xprite::prelude::*;

pub struct State {
    pub show_grid: bool,
    pub scrolling: imgui::ImVec2,
    pub xpr: Xprite,
}

impl State {
    pub fn new(xpr: Xprite) -> State {
        State {
            show_grid: true,
            scrolling: imgui::ImVec2::new(0.,0.),
            xpr
        }
    }
}