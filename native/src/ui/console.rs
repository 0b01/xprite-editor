use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_console(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    if state.show_console {
        ui.window(im_str!("Console"))
            .size((300., 200.), ImGuiCond::Appearing)
            .movable(true)
            .collapsible(false)
            .resizable(true)
            .build(|| {
                ui.text(&state.xpr.log.lock().unwrap().as_str());
            });
    }
}
