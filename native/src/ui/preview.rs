use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;

pub fn draw_preview(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) {
    ui.window(im_str!("Preview"))
        .position((LEFT_SIDE_WIDTH, 220.), ImGuiCond::Appearing)
        .size((LEFT_SIDE_WIDTH, 100.), ImGuiCond::Appearing)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {
            state.redraw_pixels(rdr).unwrap();
            let art_ratio = state.xpr.canvas.get_aspect_ratio() as f32;

            let size = match state.preview_window_state.mode {
                PreviewWindowMode::Fill => {
                    let (win_w, win_h) = ui.get_window_size();
                    let win_ratio = win_w / win_h;
                    if art_ratio > win_ratio {
                        // constrainted by width
                        [win_w * 0.9, (win_w / art_ratio) * 0.9]
                    } else {
                        // constrainted by height
                        [art_ratio * win_h * 0.8, win_h * 0.8]
                    }
                }
                PreviewWindowMode::OneX => [
                    state.xpr.canvas.art_w as f32 * 1.,
                    state.xpr.canvas.art_h as f32 * 1.,
                ],
                PreviewWindowMode::TwoX => [
                    state.xpr.canvas.art_w as f32 * 2.,
                    state.xpr.canvas.art_h as f32 * 2.,
                ],
            };

            ui.image(ImTexture::from(state.texture.unwrap()), size)
                .build();
        })
}
