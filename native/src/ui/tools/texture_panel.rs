use crate::prelude::*;
use std::rc::Rc;
use xprite::image::GenericImageView;

pub fn draw(rdr: &mut dyn Renderer, state: &mut State, ui: &Ui) {
    let tool = Rc::clone(&state.xpr_mut().toolbox.texture);
    let texture = &mut tool.borrow_mut();
    let bb = texture.get_bb();
    if bb.is_none() {
        return;
    }

    ui.tree_node(&im_str!("Orientation")).default_open(true).build(|| {
        if ui.radio_button_bool(&im_str!("reflections"), texture.orientation_reflection) {
            texture.orientation_reflection = !texture.orientation_reflection;
        }
        if ui.radio_button_bool(&im_str!("rotation"), texture.orientation_rotation) {
            texture.orientation_rotation = !texture.orientation_rotation;
        }
    });

    ui.tree_node(&im_str!("Wrap")).default_open(true).build(|| {
        if ui.radio_button_bool(&im_str!("wrap x"), texture.wrap_x) {
            texture.wrap_x = !texture.wrap_x;
        }
        if ui.radio_button_bool(&im_str!("wrap y"), texture.wrap_y) {
            texture.wrap_y = !texture.wrap_y;
        }
    });

    ui.tree_node(&im_str!("Texture size")).default_open(true).build(|| {
        ui.drag_int(&im_str!("texture width"), &mut texture.tex_w).build();
        ui.drag_int(&im_str!("texture height"), &mut texture.tex_h).build();
    });

    let mut sz = texture.pattern_size as i32;
    if ui.drag_int(&im_str!("size"), &mut sz).min(1).max(30).build() {
        texture.pattern_size = sz as u32;
    }
    if ui.is_item_hovered() {
        ui.tooltip_text("Building size of the pattern");
    }

    if ui.button(&im_str!("quilt"), [100., 20.]) {
        info!("Quilting...(this may take a few seconds)");
        match texture.finalize(&mut state.xpr_mut()) {
            Ok(img) => {
                let tex_id = rdr.add_img(img.clone(), image::RGBA(0));
                texture.tex = Some((tex_id, img));
            }
            Err(s) => {
                error!("{}", s);
            }
        }
    }

    if let Some((tex_id, img)) = &texture.tex {
        Image::new(TextureId::from(*tex_id), [100., 100.]).build(&ui); // TODO: fix hardcoded size

        if ui.button(&im_str!("finalize"), [0., 0.]) {
            let w = img.width();
            let h = img.height();
            let new_xpr = Xprite::from_img("Generated Texture".to_owned(), w, h, img.clone());
            state.push_xpr(new_xpr);
        }
    }
}
