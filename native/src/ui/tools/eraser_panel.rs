use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Brush")).build(|| {
        let brushes = BrushType::VARIANTS;
        for (_index, brush) in brushes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.eraser.borrow().brush_type == brush;
            if ui.selectable(
                im_str!("{}", brush.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0., 0.),
            ) {
                state.xpr.set_option("brush", brush.as_str()).unwrap();
            }
        }
    });
}
