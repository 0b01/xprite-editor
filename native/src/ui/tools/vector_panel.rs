use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    let tool = &mut state.xpr.toolbox.vector.borrow_mut();
    ui.drag_float(im_str!("tolerence"), &mut tool.tolerence)
        .min(1.)
        .max(50.)
        .speed(0.1)
        .build();
    ui.checkbox(im_str!("Draw Bezier"), &mut tool.draw_bezier);
    ui.checkbox(im_str!("Monotonic sort"), &mut tool.sort);
}
