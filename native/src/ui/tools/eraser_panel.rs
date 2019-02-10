use crate::prelude::*;
use super::pencil_panel::draw_brush_tree;

pub fn draw(state: &mut State, ui: &Ui) {
    let current_brush = state.xpr.toolbox.eraser.borrow().brush_type;
    draw_brush_tree(state, ui, current_brush);
}
