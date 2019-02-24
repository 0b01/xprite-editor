use crate::prelude::*;
use crate::state::preview_window::PreviewWindowMode;

pub fn draw(_rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    ui.text(im_str!("{}", state.xpr().name));

    if ui.button(im_str!("Rename Document"), (0., 0.)) {
        state.toggle_hotkeys();
        ui.open_popup(im_str!("rename_doc"));
    }

    ui.popup(im_str!("rename_doc"), || {
        let mut fname = ImString::with_capacity(100);
        fname.push_str(&state.xpr().name);
        if ui.input_text(im_str!("Filename"), &mut fname).build() {
            state.xpr_mut().set_name(fname.to_str().to_owned());
        }
        if ui.button(im_str!("done"), (0., 0.)) {
            state.toggle_hotkeys();
            ui.close_current_popup();
        }
    });

    ui.tree_node(im_str!("Document")).default_open(true).build(|| {
        let mut aspect = [0; 2];
        aspect[0] = state.xpr_mut().canvas.art_w as i32;
        aspect[1] = state.xpr_mut().canvas.art_h as i32;
        if ui.input_int2(im_str!("size"), &mut aspect).build() {
            state.xpr_mut().canvas.art_w = aspect[0] as f64;
            state.xpr_mut().canvas.art_h = aspect[1] as f64;
            state.xpr_mut().redraw = true;
        }
    });

    ui.tree_node(im_str!("Preview")).default_open(true).build(|| {
        let modes = PreviewWindowMode::VARIANTS;
        for (_index, mode) in modes.into_iter().enumerate() {
            let is_sel = &state.preview_window_state.mode == mode;
            if ui.selectable(im_str!("{}", mode.as_str()), is_sel, ImGuiSelectableFlags::empty(), (0., 0.)) {
                state.preview_window_state.mode = *mode;
            }
        }
    });
}
