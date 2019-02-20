use crate::prelude::*;
use std::f64;
use std::rc::Rc;
use xprite::image::GenericImageView;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    let tool = Rc::clone(&state.xpr_mut().toolbox.texture);
    let texture = &mut tool.borrow_mut();
    let bb = texture.get_bb();
    if bb.is_none() { return; }
    let bb = bb.unwrap();
    let (w, h) = (bb.w(), bb.h());

    let minimum_blocksize = texture.overlap * 2;
    if ui
        .drag_int(im_str!("Overlap"), &mut texture.overlap)
        .min(2)
        .max(f64::min(w, h) as i32 - 1)
        .build()
    {
        if texture.blocksize < texture.overlap * 2
            && texture.overlap * 2 < f64::min(w, h) as i32
        {
            texture.blocksize = texture.overlap * 2;
        } else {
            texture.blocksize = -1;
        }
    }
    ui.drag_int(im_str!("Block Size"), &mut texture.blocksize)
        .min(minimum_blocksize)
        .max(f64::min(w, h) as i32 - 1)
        .build();

    if ui.button(im_str!("quilt"), (100., 20.)) {
        info!("Quilting...(this may take a few seconds)");
        match texture.finalize(&mut state.xpr_mut()) {
            Ok(img) => {
                let tex_id = rdr.add_img(img.clone(), image::RGB(0));
                texture.tex = Some((tex_id, img));
            }
            Err(s) => {
                error!("{}", s);
            }
        }
    }

    if let Some((tex_id, img)) = &texture.tex {
        ui.image( ImTexture::from(*tex_id), [100., 100.]).build(); // TODO: fix hardcoded size

        if ui.button(im_str!("finalize"), (0., 0.)) {
            let w = img.width();
            let h = img.height();
            let new_xpr = Xprite::from_img("Generated Texture".to_owned(), w, h, img.clone());
            state.xprs.push(new_xpr);
        }
    }

}
