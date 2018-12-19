use crate::prelude::*;
use xprite::rendering::Renderer;


pub fn draw_tool_panel(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    let selected = &state.xpr.toolbox.tool().borrow().tool_type();
    ui
    .window(im_str!("Tool Options: {}", selected.as_str()))
    .position((sz.0 as f32 - RIGHT_SIDE_WIDTH, 20.), ImGuiCond::Always)
    .size((RIGHT_SIDE_WIDTH, (sz.1/2.) as f32), ImGuiCond::Always)
    .movable(true)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        super::tools::draw(*selected, rdr, state, ui);
    })
}
