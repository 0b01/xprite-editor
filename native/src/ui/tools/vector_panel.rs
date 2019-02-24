use crate::prelude::*;
use xprite::tools::vector;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.drag_float(im_str!("tolerence"), &mut state.xpr_mut().toolbox.vector.borrow_mut().tolerence)
        .min(1.)
        .max(50.)
        .speed(0.1)
        .build();
    ui.checkbox(im_str!("Draw Bezier"), &mut state.xpr_mut().toolbox.vector.borrow_mut().draw_bezier);
    ui.checkbox(im_str!("Monotonic sort"), &mut state.xpr_mut().toolbox.vector.borrow_mut().mono_sort);

    draw_mode(state, ui);

    if ui.button(im_str!("toggle brush"), (0., 0.)) {
        state.toggle_brush();
    }
}

fn draw_mode(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Mode")).default_open(true).build(|| {
        let modes = vector::VectorMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr_mut().toolbox.vector.borrow().mode == mode;
            if ui.selectable(im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), (0., 0.)) {
                state.xpr_mut().set_option("mode", mode.as_str()).unwrap();
            }
        }
    });
}
