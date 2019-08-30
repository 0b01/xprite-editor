use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    if ui.button(&im_str!("toggle brush"), [0., 0.]) {
        state.toggle_brush();
    }
}
