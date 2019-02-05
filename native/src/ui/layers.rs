use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("Layers"))
        .position(
            (sz.0 as f32 - RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32 + 20.),
            ImGuiCond::Always,
        )
        .size((RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32), ImGuiCond::Always)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {
            if ui.button(im_str!("+Layer"), (60., 20.)) {
                state.xpr.history.top_mut().add_layer(None);
            }
            ui.same_line(70.);
            if ui.button(im_str!("+Group"), (60., 20.)) {
                state.xpr.history.top_mut().add_group(None);
            }

            let ngroups: Vec<_> = {
                let top = state.xpr.history.top_mut();
                if top.selected_layer_mut().is_none() {
                    return;
                }
                top.groups.iter().map(|i|i.1.len()).collect()
            };

            for (group_id, group_len) in ngroups.into_iter().enumerate() {

                macro_rules! group {
                    () => {
                        state.xpr.history.top_mut().groups[group_id]
                    }
                };

                ui.with_id(group_id as i32, || {
                    ui.tree_node(im_str!("{}", &group!().0))
                        .default_open(true)
                        .open_on_double_click(false)
                        .build(||{
                            if ui.is_item_hovered()
                            && ui.imgui().is_mouse_clicked(ImMouseButton::Right)
                            {
                                info!("clicked group");
                                // change selected group
                                state.xpr.switch_layer(group_id, 0);
                                // disable hotkeys
                                state.toggle_hotkeys();
                                ui.open_popup(im_str!("Rename Group"));
                            }
                            ui.popup_modal(im_str!("Rename Group"))
                                .inputs(true)
                                .collapsible(false)
                                .resizable(false)
                                .movable(false)
                                .build(|| {
                                    let name = state.xpr
                                        .history.top()
                                        .selected_group().unwrap()
                                        .0.to_owned();
                                    let mut im = ImString::new(name);
                                    ui.with_item_width(100., || {
                                        if ui
                                            .input_text(im_str!(""), &mut im)
                                            .auto_select_all(true)
                                            .enter_returns_true(true)
                                            .build()
                                        {
                                            let im: &str = &im.as_ref();
                                            info!("renaming: {}", im);
                                            // state.xpr.rename_layer(&im).unwrap();
                                            state.xpr
                                                .history.top_mut()
                                                .selected_group_mut().unwrap()
                                                .0 = im.to_owned();
                                            state.toggle_hotkeys();
                                            ui.close_current_popup();
                                        }
                                    });
                                });

                            for i in 0..group_len {
                                let is_sel = {
                                    let top = state.xpr.history.top();
                                    top.selected == i && top.sel_group == group_id
                                };

                                macro_rules! layer {
                                    () => {
                                        group!().1[i]
                                    }
                                }

                                {
                                    let name = layer!().name.as_str();
                                    if ui.selectable(
                                        im_str!("{}", name),
                                        is_sel,
                                        ImGuiSelectableFlags::AllowDoubleClick,
                                        (100., 0.),
                                    ) {
                                        if ui.is_item_hovered()
                                        && ui.imgui().is_mouse_clicked(ImMouseButton::Right)
                                        {
                                            info!("double clicked");

                                            // disable hotkeys
                                            state.toggle_hotkeys();
                                            ui.open_popup(im_str!("Rename Layer"));
                                        }

                                        ui.popup_modal(im_str!("Rename Layer"))
                                            .inputs(true)
                                            .collapsible(false)
                                            .resizable(false)
                                            .movable(false)
                                            .build(|| {
                                                let name = { state.xpr.current_layer().unwrap().name.to_owned() };
                                                let mut im = ImString::new(name);
                                                ui.with_item_width(100., || {
                                                    if ui
                                                        .input_text(im_str!(""), &mut im)
                                                        .auto_select_all(true)
                                                        .enter_returns_true(true)
                                                        .build()
                                                    {
                                                        let im: &str = &im.as_ref();
                                                        info!("renaming: {}", im);
                                                        state.xpr.rename_layer(&im).unwrap();
                                                        state.toggle_hotkeys();
                                                        ui.close_current_popup();
                                                    }
                                                });
                                            });

                                        state.xpr.switch_layer(group_id, i);
                                    }
                                }

                                ui.same_line(100.);
                                ui.with_id(i as i32, || {
                                    if ui.checkbox(im_str!(""), &mut layer!().visible) {
                                        layer!().visible = !layer!().visible; // undo imgui checkbox mutation
                                        state.xpr.toggle_layer_visibility(group_id, i).unwrap(); // enter history frame and toggle
                                    }
                                });
                                ui.same_line(140.);
                                ui.with_id(i as i32, || {
                                    if ui.button(im_str!("X"), (20., 20.)) {
                                        state.xpr.remove_layer(group_id, i).unwrap();
                                    }
                                });
                            }
                        });
                });
            }
        })
}