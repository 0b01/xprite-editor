use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_palette(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui
    .window(im_str!("Palette"))
    .position((0.,220.), ImGuiCond::Appearing)
    .size((LEFT_SIDE_WIDTH, 800.), ImGuiCond::Appearing)
    .movable(false)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        let tools = ToolType::VARIANTS;

        for (_index, name) in tools.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.tool().borrow().tool_type() == name;
            if ui.selectable(
                im_str!("{}", name.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0.,0.)
            ) {
                state.xpr.change_tool(name);
            }
        }
    })
}
