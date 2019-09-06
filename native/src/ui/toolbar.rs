use crate::prelude::*;
// use xprite::prelude::*;
// use xprite::rendering::Renderer;

pub fn draw_toolbar(state: &mut State, ui: &Ui) {
    ui.window(&im_str!("Toolbox"))
        .no_bring_to_front_on_focus(true)
        .position([0., 20.], Condition::Appearing)
        .size([LEFT_SIDE_WIDTH, 200.], Condition::Appearing)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(|| {
            let selected = state.xpr_mut().toolbox.selected.clone();
            let tools = ToolType::VARIANTS;
            for (_index, name) in tools.iter().enumerate() {
                let is_sel = selected == *name;
                if ui.selectable(&im_str!("{}", name.as_str()), is_sel, ImGuiSelectableFlags::empty(), [0., 0.]) {
                    state.xpr_mut().change_tool(*name).unwrap();
                }
            }
        })
}
