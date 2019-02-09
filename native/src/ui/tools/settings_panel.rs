use crate::prelude::*;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    ui.tree_node(im_str!("Document"))
        .default_open(true)
        .build(|| {
            let mut aspect = [0; 2];
            aspect[0] = state.xpr.canvas.art_w as i32;
            aspect[1] = state.xpr.canvas.art_h as i32;
            if ui.input_int2(im_str!("size"), &mut aspect).build() {
                state.xpr.canvas.art_w = aspect[0] as f64;
                state.xpr.canvas.art_h = aspect[1] as f64;
                state.xpr.redraw = true;
            }
        });

    ui.tree_node(im_str!("Preview"))
        .default_open(true)
        .build(|| {
            let modes = PreviewWindowMode::VARIANTS;
            for (_index, mode) in modes.into_iter().enumerate() {
                let is_sel = &state.preview_window_state.mode == mode;
                if ui.selectable(
                    im_str!("{}", mode.as_str()),
                    is_sel,
                    ImGuiSelectableFlags::empty(),
                    (0., 0.),
                ) {
                    state.preview_window_state.mode = *mode;
                }
            }
        });
}
