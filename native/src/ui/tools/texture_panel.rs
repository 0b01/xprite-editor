use std::rc::Rc;
use crate::prelude::*;

pub fn draw(state: &mut State, ui: &Ui) {
    let tool = Rc::clone(&state.xpr.toolbox.texture);
    let texture = &mut tool.borrow_mut();

    let minimum_blocksize = texture.overlap * 2;
    if ui.drag_int(im_str!("Overlap"), &mut texture.overlap)
        .min(1).max(100)
        .build() {
        if texture.blocksize < texture.overlap * 2 {
            texture.blocksize = texture.overlap * 2;
        }
    }
    ui.drag_int(im_str!("Block Size"), &mut texture.blocksize)
        .min(minimum_blocksize).max(100)
        .build();

    if ui.button(im_str!("Process"), (100., 30.)) {
        info!("Processing");
        texture.finalize(&mut state.xpr);
    }
}