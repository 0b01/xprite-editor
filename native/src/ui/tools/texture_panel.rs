use std::rc::Rc;
use std::f32;
use crate::prelude::*;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    let tool = Rc::clone(&state.xpr.toolbox.texture);
    let texture = &mut tool.borrow_mut();
    let dims = texture.get_dims();
    if dims.is_none() { return; }
    let (x, y, _) = dims.unwrap();

    let minimum_blocksize = texture.overlap * 2;
    if ui.drag_int(im_str!("Overlap"), &mut texture.overlap)
        .min(1)
        .max(f32::min(x, y) as i32 - 1)
        .build() {
        if texture.blocksize < texture.overlap * 2 {
            texture.blocksize = texture.overlap * 2;
        }
    }
    ui.drag_int(im_str!("Block Size"), &mut texture.blocksize)
        .min(minimum_blocksize)
        .max(f32::min(x, y) as i32)
        .build();

    if ui.button(im_str!("Process"), (100., 30.)) {
        info!("Processing");
        if let Ok(img) = texture.finalize(&mut state.xpr) {
            texture.current_id = Some(rdr.add_img(img));
        }
    }
    if let Some(texture_id) = texture.current_id {
        ui.image(
            ImTexture::from(texture_id),
            [100., 100.]
        ).build();
    }

}