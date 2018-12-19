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
        .min(2)
        .max(f32::min(x, y) as i32 - 1)
        .build() {
        if texture.blocksize < texture.overlap * 2 && texture.overlap * 2 < f32::min(x,y) as i32 {
            texture.blocksize = texture.overlap * 2;
        } else {
            texture.blocksize = -1;
        }
    }
    ui.drag_int(im_str!("Block Size"), &mut texture.blocksize)
        .min(minimum_blocksize)
        .max(f32::min(x, y) as i32 - 1)
        .build();

    if ui.button(im_str!("Quilt!"), (100., 20.)) {
        info!("Quilting...(this may take a few seconds)");
        match texture.finalize(&mut state.xpr) {
            Ok(img) => {texture.current_id = Some(rdr.add_img(img));}
            Err(s) => { error!("{}", s); }
        }
    }
    if let Some(texture_id) = texture.current_id {
        ui.image(
            ImTexture::from(texture_id),
            [100., 100.]
        ).build();
    }

}