use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    let mut tool = state.xpr.toolbox.ellipse.borrow_mut();
    if ui.checkbox(im_str!("filled"), &mut tool.filled) {}
}
