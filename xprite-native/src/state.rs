use xprite::prelude::*;

pub struct State<'a> {
    pub show_grid: bool,
    pub scrolling: imgui::ImVec2,
    pub xpr: Xprite<'a>,
}

impl<'a> State<'a> {
    pub fn new(xpr: Xprite<'a>) -> State<'a> {
        State {
            show_grid: true,
            scrolling: imgui::ImVec2::new(0.,0.),
            xpr
        }
    }
}