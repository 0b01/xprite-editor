use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_console(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    if state.show_console {
        ui
        .window(im_str!("Console"))
        .size((300., 200.), ImGuiCond::Always)
        .movable(true)
        .collapsible(false)
        .resizable(true)
        .build(|| {
            ui.text(&state.xpr.log);
        });
    }
}
