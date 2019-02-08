use crate::prelude::*;

pub fn draw(rdr: &mut Renderer, state: &mut State, ui: &Ui) {
    ui.text("Settings");
    let mut aspect = [0; 2];
    aspect[0] = state.xpr.canvas.art_w as i32;
    aspect[1] = state.xpr.canvas.art_h as i32;
    if ui.input_int2(im_str!("size"), &mut aspect).build() {
        state.xpr.canvas.art_w = aspect[0] as f64;
        state.xpr.canvas.art_h = aspect[1] as f64;
        state.xpr.redraw = true;
    }
}
