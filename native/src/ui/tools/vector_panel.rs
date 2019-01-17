use crate::prelude::*;
use xprite::tools::vector;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.drag_float(
        im_str!("tolerence"),
        &mut state.xpr.toolbox.vector.borrow_mut().tolerence
    )
        .min(1.)
        .max(50.)
        .speed(0.1)
        .build();
    ui.checkbox(
        im_str!("Draw Bezier"),
        &mut state.xpr.toolbox.vector.borrow_mut().draw_bezier
    );
    ui.checkbox(
        im_str!("Monotonic sort"),
        &mut state.xpr.toolbox.vector.borrow_mut().mono_sort
    );

    draw_brush(state, ui);
    draw_mode(state, ui);
}

fn draw_brush(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Brush")).build(|| {
        let brushes = BrushType::VARIANTS;
        for (_index, brush) in brushes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.vector.borrow().brush_type == brush;
            if ui.selectable(
                im_str!("{}", brush.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0., 0.),
            ) {
                state.xpr.set_option("brush", brush.as_str()).unwrap();
            }
        }
    });
}

fn draw_mode(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Mode")).build(|| {
        let modes = vector::VectorMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.vector.borrow().mode == mode;
            if ui.selectable(
                im_str!("{}", mode.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0., 0.),
            ) {
                state.xpr.set_option("mode", mode.as_str()).unwrap();
            }
        }
    });
}