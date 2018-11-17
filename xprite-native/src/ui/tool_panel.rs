use crate::prelude::*;
use xprite::rendering::Renderer;


pub fn tool_panel(_rdr: &Renderer, state: &mut State, ui: &Ui) { let sz = ui.frame_size().logical_size;
    let selected = &state.xpr.toolbox.tool().borrow().tool_type();
    ui
    .window(im_str!("Tool Options: {}", selected.as_str()))
    .position((sz.0 as f32 - 300., 20.), ImGuiCond::Appearing)
    .size((300., sz.1 as f32 - 20.), ImGuiCond::Appearing)
    .movable(true)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        super::tools::draw(&selected, state, ui);
    })
}
