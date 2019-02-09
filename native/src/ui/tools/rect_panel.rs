use crate::prelude::*;
use std::f64::consts::PI;

pub fn draw(state: &mut State, ui: &Ui) {
    let mut tool = state.xpr.toolbox.rect.borrow_mut();
    if ui.checkbox(im_str!("filled"), &mut tool.filled) {}

    if let Some(info) = tool.get_info() {
        ui.tree_node(im_str!("Status"))
            .default_open(true)
            .build(|| {
                ui.text(im_str!("Aspect ratio: {:.2}", info.aspect_ratio));
                ui.text(im_str!("Angle: {:.2}", info.angle * 2. * PI));
                ui.text(im_str!("p0: {:?}", info.top_left));
                ui.text(im_str!("p1: {:?}", info.bottom_right));
                ui.text(im_str!("width: {}", info.width));
                ui.text(im_str!("height: {}", info.height));
            });
    }
}
