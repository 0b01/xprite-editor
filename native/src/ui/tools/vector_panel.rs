use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.drag_float(im_str!("tolerence"), &mut state.xpr.toolbox.vector.borrow_mut().tolerence)
        .min(1.)
        .max(50.)
        .speed(0.1)
        .build();
}