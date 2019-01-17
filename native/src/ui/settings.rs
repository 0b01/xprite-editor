use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_settings(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    if !state.show_settings {
        return;
    }
    ui.window(im_str!("Settings")).build(|| {})
}
