use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui
    .window(im_str!("Layers"))
    .position((sz.0 as f32 - 300., (sz.1 / 2.) as f32 + 20.), ImGuiCond::Appearing)
    .size((300., (sz.1 / 2.) as f32), ImGuiCond::Appearing)
    .movable(true)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        if ui.button(im_str!("new"), (100.,30.)) {
            state.xpr.history.top_mut().add(None);
        }
        let layer_manager = &mut state.xpr.history.top_mut();
        for (i, layer) in layer_manager.layers.iter().enumerate() {
            {
                let layer_ref = layer.borrow();
                let name = layer_ref.name.as_str();
                let is_sel = layer_manager.is_selected(&layer);
                if ui.selectable(
                    im_str!("{}", name),
                    is_sel,
                    ImGuiSelectableFlags::empty(),
                    (50.,0.)
                ) {
                    layer_manager.selected_layer = layer.clone();
                }
            }

            {
                ui.same_line(100.);
                ui.with_id(i as i32, || {
                    let mut layer_ref = layer.borrow_mut();
                    ui.checkbox(im_str!(""), &mut layer_ref.visible);
                })
            }
        }
    })
}
