use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui
    .window(im_str!("Layers"))
    .position((sz.0 as f32 - RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32 + 20.), ImGuiCond::Always)
    .size((RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32), ImGuiCond::Always)
    .movable(true)
    .collapsible(true)
    .resizable(true)
    .build(|| {
        if ui.button(im_str!("+Layer"), (20.,20.)) {
            state.xpr.history.top_mut().add_layer(None);
        }
        if ui.button(im_str!("+Group"), (20.,20.)) {
            state.xpr.history.top_mut().add_group(None);
        }

        ui.popup_modal(im_str!("Rename Layer"))
          .inputs(true)
          .collapsible(false)
          .resizable(false)
          .movable(false)
          .build(|| {
            let name = {
                state.xpr.current_layer().unwrap().name.to_owned()
            };
            let mut im = ImString::new(name);
            ui.with_item_width(100., ||{
                if ui
                .input_text(im_str!(""), &mut im)
                .auto_select_all(true)
                .enter_returns_true(true)
                .build() {
                    let name : &str = im.as_ref();
                    info!("renaming: {}", name);
                    state.xpr.rename_layer(&im.as_ref()).unwrap();
                    state.toggle_hotkeys();
                    ui.close_current_popup();
                }
            });
        });

        let (mut groups, selected_layer) = {
            let top = state.xpr.history.top_mut();
            if top.selected_layer_mut().is_none() {return;}
            let selected_layer = top.selected_layer_mut().unwrap().clone();
            (
                top.groups.clone(),
                selected_layer
            )
        };

        for (group_id, group) in groups.iter_mut().enumerate() {
            draw_group_line(state, ui, group_id, group);
            for (i, layer) in group.1.iter_mut().enumerate() {

                let is_sel = layer == &selected_layer;
                draw_layer_line(state, ui, group_id, i, layer, is_sel);
            }
        }
    })
}

fn draw_group_line(state: &mut State, ui: &Ui, group_id: usize, group: &mut (String, Vec<Layer>)) {
    ui.text(im_str!("{}", group.0));
}

fn draw_layer_line(state: &mut State, ui: &Ui, group_id: usize, i: usize, layer: &mut Layer, is_sel: bool) {
        {
            // one layer
            let name = layer.name.as_str();
            if ui.selectable(
                im_str!("{}", name),
                is_sel,
                ImGuiSelectableFlags::AllowDoubleClick,
                (50.,0.)
            ) {
                if ui.imgui().is_mouse_double_clicked(imgui::ImMouseButton::Left) {
                    info!("double clicked");

                    // diable hotkeys
                    state.toggle_hotkeys();
                    ui.open_popup(im_str!("Rename Layer"));
                }
                state.xpr.switch_layer(group_id, i);
            }
        }

    ui.with_id(group_id as i32, || {
        ui.same_line(100.);
        ui.with_id(i as i32, || {
            if ui.checkbox(im_str!(""), &mut layer.visible) {
                layer.visible = !layer.visible; // undo imgui checkbox mutation
                state.xpr.toggle_layer_visibility(group_id, i).unwrap(); // enter history frame and toggle
            }
        });
        ui.same_line(140.);
        ui.with_id(i as i32, || {
            if ui.button(im_str!("X"), (20.,20.)) {
                state.xpr.remove_layer(group_id, i).unwrap();
            }
        });
    });
}