use super::pencil_panel::draw_brush_tree;
use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    let current_brush = state.xpr_mut().toolbox.eraser.borrow().brush_type;
    draw_brush_tree(state, ui, current_brush);
}
