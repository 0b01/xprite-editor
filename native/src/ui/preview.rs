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
            let new_hash = state.xpr.img_hash();
            match state.preview_texture {
                None => {
                    update_preview(rdr, state, ui, new_hash);
                }
                Some((old_hash, _)) => {
                    if old_hash != new_hash {
                        // trace!("updating preview");
                        update_preview(rdr, state, ui, new_hash);
                    }
                }
            };
            ui.image(
                ImTexture::from(state.preview_texture.unwrap().1),
                [state.xpr.canvas.art_w as f32 * 3., state.xpr.canvas.art_h as f32 * 3.],
            )
            .build();
        })
}

fn update_preview(rdr: &mut Renderer, state: &mut State, _ui: &Ui, new_hash: u64) {
    let mut img_rdr = ImageRenderer::new(state.xpr.canvas.art_w, state.xpr.canvas.art_h);
    state.xpr.preview(&mut img_rdr).unwrap();
    img_rdr.render();
    let img = img_rdr.img();
    state.preview_texture = Some((new_hash, rdr.add_img(img.to_owned(), image::RGBA(0))));
}
