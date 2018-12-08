use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_palette(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui
    .window(im_str!("Palette"))
    .position((0.,220.), ImGuiCond::Appearing)
    .size((LEFT_SIDE_WIDTH, 800.), ImGuiCond::Appearing)
    .movable(false)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        let mut sel: [f32; 4] = state.xpr.selected_color.into();
        println!("{:?}", sel);
        if ui.color_picker(im_str!("color"), &mut sel).build() {
            println!("{:?}", sel);
            let sel = [sel[0] * 255., sel[1] * 255., sel[2] * 255., sel[3]];
            println!("{:?}", sel);
            state.xpr.selected_color = sel.into();
            println!("-----------------");
        }


    })
}
