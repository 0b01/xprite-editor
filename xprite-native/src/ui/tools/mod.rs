use crate::prelude::*;

pub mod pencil_panel;
pub mod line_panel;
pub mod paintbucket_panel;

pub fn draw(selected: &ToolType, state: &mut State, ui: &Ui) {
    match selected {
        ToolType::Pencil => pencil_panel::draw(state, ui),
        ToolType::Line => line_panel::draw(state, ui),
        ToolType::PaintBucket => paintbucket_panel::draw(state, ui),
    }
}