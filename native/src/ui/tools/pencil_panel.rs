use crate::prelude::*;
use xprite::tools::pencil::PencilMode;

pub fn draw(state: &mut State, ui: &Ui) {
    ui.tree_node(&im_str!("Mode")).default_open(true).build(|| {
        for (_index, mode) in PencilMode::VARIANTS.iter().enumerate() {
            let is_sel = &state.xpr_mut().toolbox.pencil.borrow().mode == mode;
            if ui.selectable(&im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), [0., 0.]) {
                state.xpr_mut().set_option("mode", mode.as_str()).unwrap();
            }
        }
    });

    if state.xpr_mut().toolbox.pencil.borrow().mode == PencilMode::SelectiveAntiAliasing {
        let mut pencil = state.xpr_mut().toolbox.pencil.borrow_mut();
        let mut sel: [f32; 4] = unsafe { pencil.aa_alt_color.as_rgba().into() };
        let id = im_str!("MyColor##{}", "background");
        let misc_flags = {
            let mut f = ImGuiColorEditFlags::empty();
            f.set(ImGuiColorEditFlags::HDR, true);
            f.set(ImGuiColorEditFlags::AlphaPreview, true);
            f.set(ImGuiColorEditFlags::NoOptions, false);
            f.set(ImGuiColorEditFlags::NoInputs, true);
            f.set(ImGuiColorEditFlags::NoLabel, true);
            f.set(ImGuiColorEditFlags::NoPicker, false);
            f
        };
        let b = ui.color_edit(&id, &mut sel).flags(misc_flags).alpha(false);
        if b.build() {
            pencil.aa_alt_color = sel.into();
        }
    }

    if ui.button(&im_str!("toggle brush"), [0., 0.]) {
        state.toggle_brush();
    }
}
