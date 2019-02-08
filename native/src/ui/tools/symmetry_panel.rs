use crate::prelude::*;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    if ui.button(im_str!("+"), (0., 0.)) {
        let symm = state.xpr.toolbox.symmetry.borrow_mut();
        // symm.add()
    }
}
