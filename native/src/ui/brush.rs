use crate::prelude::*;

pub fn draw_brush(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    if !state.show_brush {
        return;
    }

    let sz = ui.io().display_size;
    Window::new(&im_str!("Brush"))
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH * 2., 20.], Condition::Once)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 2.) as f32], Condition::Once)
        .bring_to_front_on_focus(true)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(&ui, || {
            let current_tool = state.xpr_mut().toolbox.selected;
            let brush_type = if let Some(b) = state.xpr().get_brush_for_tool(current_tool) {
                b.brush_type
            } else if let Some(b) = state.xpr().get_brush_for_tool(state.xpr().last_tool()) {
                b.brush_type
            } else {
                return;
            };
            draw_brush_tree(state, ui, brush_type, current_tool);
        });
}

pub fn draw_brush_tree(state: &mut State, ui: &Ui, current_brush: BrushType, tool_type: ToolType) {
    ui.tree_node(&im_str!("Brush")).default_open(true).build( || {
        for (_index, brush) in BrushType::VARIANTS.iter().enumerate() {
            let is_sel = &current_brush == brush;
            if Selectable::new(&im_str!("{}", brush.as_str()))
                .selected(is_sel)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui)
            {
                state.set_brush_for_tool(*brush, tool_type);
            }
        }
    });

    ui.tree_node(&im_str!("Brush Settings")).default_open(true).build( || {
        if ui.drag_int(&im_str!("size"), &mut state.brush.sz[0]).build() {
            state.set_brush_for_tool(current_brush, tool_type);
        }
        if ui.drag_int(&im_str!("angle"), &mut state.brush.sz[1]).build() {
            state.set_brush_for_tool(current_brush, tool_type);
        }
        macro_rules! angle_btn {
            ($angle: literal) => {
                if ui.button(&im_str!("{}", stringify!($angle)), [0., 0.]) {
                    state.brush.sz[1] = $angle;
                    state.set_brush_for_tool(current_brush, tool_type);
                }
            };
        }

        angle_btn!(30);

        ui.same_line(0.);
        angle_btn!(45);

        ui.same_line(0.);
        angle_btn!(60);

        ui.same_line(0.);
        angle_btn!(90);

        angle_btn!(120);

        ui.same_line(0.);
        angle_btn!(135);

        ui.same_line(0.);
        angle_btn!(150);
    });
}
