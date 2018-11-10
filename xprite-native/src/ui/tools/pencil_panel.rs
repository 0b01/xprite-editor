use crate::prelude::*;
use xprite::tools::pencil;

pub fn draw(state: &mut State, ui: &Ui) {
    ui
    .tree_node(im_str!("Mode"))
    .build(|| {
        let modes = pencil::PencilMode::VARIANTS;
        for (_index, mode) in modes.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.pencil.borrow().mode == mode;
            if ui.selectable(
                im_str!("{}", mode.as_str()),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0.,0.)
            ) {
                state.xpr.set_option("mode", mode.as_str());
            }
        }

    });
}