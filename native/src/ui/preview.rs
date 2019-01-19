use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_preview(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    ui.window(im_str!("Preview"))
        .position((LEFT_SIDE_WIDTH, 220.), ImGuiCond::Appearing)
        .size((LEFT_SIDE_WIDTH, 100.), ImGuiCond::Appearing)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {

            state.redraw_pixels(rdr).unwrap();

            ui.image(
                ImTexture::from(state.preview_texture.unwrap()),
                [state.xpr.canvas.art_w as f32 * 3., state.xpr.canvas.art_h as f32 * 3.],
            )
            .build();
        })
}
