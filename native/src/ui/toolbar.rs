use crate::prelude::*;
// use xprite::prelude::*;
// use xprite::rendering::Renderer;

pub fn draw_toolbar(state: &mut State, ui: &Ui) {
    ui.window(im_str!("toolbox"))
        .position((0., 20.), ImGuiCond::Appearing)
        .size((LEFT_SIDE_WIDTH, 200.), ImGuiCond::Appearing)
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
                    (0., 0.),
                ) {
                    state.xpr.change_tool(*name).unwrap();
                }
            }
        })
}
