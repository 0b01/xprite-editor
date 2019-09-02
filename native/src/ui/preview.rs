use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use crate::state::preview_window::PreviewWindowMode;

pub fn draw_preview(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) {
    ui.window(&im_str!("Preview"))
        .position([LEFT_SIDE_WIDTH, 220.], Condition::Appearing)
        .size([LEFT_SIDE_WIDTH, 100.], Condition::Appearing)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {
            state.redraw_pixels(rdr).unwrap();
            let art_ratio = state.xpr_mut().canvas.get_aspect_ratio() as f32;

            let size = match state.preview_window_state.mode {
                PreviewWindowMode::Fill => {
                    let sz = ui.get_window_size();
                    let win_w = sz[0];
                    let win_h = sz[1];
                    let win_ratio = win_w / win_h;
                    if art_ratio > win_ratio {
                        // constrainted by width
                        [win_w * 0.9, (win_w / art_ratio) * 0.9]
                    } else {
                        // constrainted by height
                        [art_ratio * win_h * 0.8, win_h * 0.8]
                    }
                }
                PreviewWindowMode::OneX => [state.xpr_mut().canvas.art_w as f32 * 1., state.xpr_mut().canvas.art_h as f32 * 1.],
                PreviewWindowMode::TwoX => [state.xpr_mut().canvas.art_w as f32 * 2., state.xpr_mut().canvas.art_h as f32 * 2.],
            };

            ui.image(TextureId::from(state.texture.unwrap()), size).build();
        })
}
