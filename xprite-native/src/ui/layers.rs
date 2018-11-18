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

        if ui.button(im_str!("add"), (60.,20.)) {
            state.xpr.history.top_mut().add(None);
        }

        let (layers, selected_layer) = {
            let layer_manager = state.xpr.history.top_mut();
            let selected_layer = layer_manager.selected_layer.clone();
            (
                layer_manager.layers.clone(),
                selected_layer
            )
        };
        for (i, layer) in layers.iter().enumerate() {
            {
                let layer_ref = layer.borrow();
                let name = layer_ref.name.as_str();
                let is_sel = layer == &selected_layer;
                if ui.selectable(
                    im_str!("{}", name),
                    is_sel,
                    ImGuiSelectableFlags::empty(),
                    (50.,0.)
                ) {
                    state.xpr.history.top_mut().selected_layer = layer.clone();
                }
            }

            ui.same_line(100.);
            ui.with_id(i as i32, || {
                let mut layer_ref = layer.borrow_mut();
                if ui.checkbox(im_str!(""), &mut layer_ref.visible) {
                    layer_ref.visible = !layer_ref.visible; // undo imgui checkbox mutation
                    drop(layer_ref); // drop borrow
                    state.xpr.toggle_visible(&layer); // enter history frame and toggle
                }
            })

        }
    })
}
