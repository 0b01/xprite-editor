use crate::prelude::*;
use xprite::tools::pencil::PencilMode;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Mode")).default_open(true).build(|| {
        for (_index, mode) in PencilMode::VARIANTS.iter().enumerate() {
            let is_sel = &state.xpr_mut().toolbox.pencil.borrow().mode == mode;
            if ui.selectable(im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), (0., 0.)) {
                state.xpr_mut().set_option("mode", mode.as_str()).unwrap();
            }
        }
    });

    if ui.button(im_str!("toggle brush"), (0., 0.)) {
        state.toggle_brush();
    }
}
