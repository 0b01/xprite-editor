use crate::prelude::*;
use std::rc::Rc;
use xprite::tools::symmetry::SymmetryMode;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    let symm = Rc::clone(&state.xpr.toolbox.symmetry);
    let mut tool = symm.borrow_mut();
    if ui.button(im_str!("+"), (0.,0.)) {
        tool.steps.push(SymmetryMode::Horizontal(1.));
    }

    let len = tool.steps.len();
    for i in 0..len {
        let symm_mode = &tool.steps[i];
        ui.text(symm_mode.as_str());
    }
}
