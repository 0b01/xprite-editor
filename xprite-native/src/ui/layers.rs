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

        if ui.button(im_str!("+"), (20.,20.)) {
            state.xpr.history.top_mut().add(None);
        }

        ui.popup_modal(im_str!("Rename Layer"))
          .inputs(true)
          .collapsible(false)
          .resizable(false)
          .movable(false)
          .build(|| {
            let name = {
                let layers = state.xpr.history.top_mut();
                layers.selected_layer.borrow_mut().name.to_owned()
            };
            let mut im = ImString::new(name);
            ui.with_item_width(100., ||{
                if ui
                .input_text(im_str!(""), &mut im)
                .auto_select_all(true)
                .enter_returns_true(true)
                .build() {
                    let name : &str = im.as_ref();
                    info!("renameing: {}", name);
                    state.xpr.rename_layer(&im.as_ref());
                    ui.close_current_popup();
                }
            });
        });

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
                    ImGuiSelectableFlags::AllowDoubleClick,
                    (50.,0.)
                ) {
                    if ui.imgui().is_mouse_double_clicked(imgui::ImMouseButton::Left) {
                        info!("double clicked");
                        ui.open_popup(im_str!("Rename Layer"));
                    }
                    state.xpr.history.top_mut().selected_layer = layer.clone();
                }
            }

            ui.same_line(100.);
            ui.with_id(i as i32, || {
                let mut layer_ref = layer.borrow_mut();
                if ui.checkbox(im_str!(""), &mut layer_ref.visible) {
                    layer_ref.visible = !layer_ref.visible; // undo imgui checkbox mutation
                    drop(layer_ref); // drop borrow
                    state.xpr.toggle_layer_visibility(&layer); // enter history frame and toggle
                }
            });
            ui.same_line(140.);
            ui.with_id(i as i32, || {
                if ui.button(im_str!("X"), (20.,20.)) {
                    state.xpr.remove_layer(&layer);
                }
            });
        }
    })
}
