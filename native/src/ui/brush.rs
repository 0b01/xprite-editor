use crate::prelude::*;

pub fn draw_brush(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    if !state.show_brush {
        return;
    }

    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("Brush"))
        .position((sz.0 as f32 - RIGHT_SIDE_WIDTH * 2., 20.), ImGuiCond::Once)
        .size((RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32), ImGuiCond::Once)
        .no_bring_to_front_on_focus(false)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {
            let current_tool = state.xpr_mut().toolbox.selected;
            let current_brush = match current_tool {
                ToolType::Pencil => state.xpr_mut().toolbox.pencil.borrow().brush_type,
                ToolType::Eraser => state.xpr_mut().toolbox.eraser.borrow().brush_type,
                ToolType::Vector => state.xpr_mut().toolbox.vector.borrow().brush_type,
                _ => return,
            };
            draw_brush_tree(state, ui, current_brush, current_tool);
        });
}

pub fn draw_brush_tree(state: &mut State, ui: &Ui, current_brush: BrushType, tool_type: ToolType) {
    ui.tree_node(im_str!("Brush")).default_open(true).build(|| {
        for (_index, brush) in BrushType::VARIANTS.iter().enumerate() {
            let is_sel = &current_brush == brush;
            if ui.selectable(im_str!("{}", brush.as_str()), is_sel, ImGuiSelectableFlags::empty(), (0., 0.)) {
                set_brush_for_tool(state, *brush, tool_type);
            }
        }
    });

    ui.tree_node(im_str!("Brush Settings")).default_open(true).build(|| {
        if ui.drag_int(im_str!("size"), &mut state.brush.sz[0]).build() {
            set_brush_for_tool(state, current_brush, tool_type);
        }
        if ui.drag_int(im_str!("angle"), &mut state.brush.sz[1]).build() {
            set_brush_for_tool(state, current_brush, tool_type);
        }
        macro_rules! angle_btn {
            ($angle: literal) => {
                if ui.button(im_str!("{}", stringify!($angle)), (0., 0.)) {
                    state.brush.sz[1] = $angle;
                    set_brush_for_tool(state, current_brush, tool_type);
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

pub fn set_brush_for_tool(state: &mut State, brush: BrushType, tool_type: ToolType) {
    macro_rules! tool {
        ($tool: expr) => {
            match brush {
                BrushType::Pixel | BrushType::Cross => {
                    $tool.set(state.xpr_mut(), "brush", brush.as_str()).unwrap();
                }
                BrushType::Circle | BrushType::Square => {
                    let sz = state.brush.sz[0];
                    $tool.set(state.xpr_mut(), "brush", &format!("{}{}", brush.as_str(), sz)).unwrap();
                }
                BrushType::Line => {
                    let sz0 = state.brush.sz[0];
                    let sz1 = state.brush.sz[1];
                    $tool
                        .set(state.xpr_mut(), "brush", &format!("{}{},{}", brush.as_str(), sz0, sz1))
                        .unwrap();
                }
            };
        };
    }

    match tool_type {
        ToolType::Pencil => tool!(state.xpr_mut().toolbox.pencil.clone().borrow_mut()),
        ToolType::Vector => tool!(state.xpr_mut().toolbox.vector.clone().borrow_mut()),
        ToolType::Eraser => tool!(state.xpr_mut().toolbox.eraser.clone().borrow_mut()),
        _ => return,
    }
}
