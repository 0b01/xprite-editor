/// TODO: this file is hideous
use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    ui.window(&im_str!("Layers"))
        .no_bring_to_front_on_focus(true)
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH, (sz[1] / 2.) as f32 + 20.], Condition::Always)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 2.) as f32], Condition::Always)
        .movable(true)
        .collapsible(true)
        .resizable(true)
        .build(|| {
            if ui.button(&im_str!("+Layer"), [60., 20.]) {
                let visible = true;
                state.xpr_mut().frame_mut().add_layer(None, visible);
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Add layer under selected group");
            }
            ui.same_line(0.);
            if ui.button(&im_str!("+Group"), [60., 20.]) {
                state.xpr_mut().frame_mut().add_group(None);
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Add group");
            }

            let ngroups: Vec<_> = {
                let frame = state.xpr_mut().frame_mut();
                if frame.layer().is_none() {
                    return;
                }
                frame.groups.iter().map(|i| i.1.len()).collect()
            };

            for (group_id, n_layers) in ngroups.clone().into_iter().enumerate() {
                ui.with_id(group_id as i32, || {
                    macro_rules! group {
                        () => {
                            state.xpr_mut().frame_mut().groups[group_id]
                        };
                    };

                    ui.tree_node(&im_str!("{}", &group!().0))
                        .default_open(true)
                        .open_on_double_click(false)
                        .build(|| {
                            // right click
                            if (ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Right))
                                || (state.rename_group.is_some() && state.rename_group.unwrap() == group_id)
                            {
                                info!("clicked group");
                                // change selected group
                                state.xpr_mut().switch_layer(group_id, 0);
                                ui.open_popup(&im_str!("contextmenu_group##{}", group_id));
                            }
                            ui.popup(&im_str!("contextmenu_group##{}", group_id), || {
                                if state.rename_group.is_some() {
                                    let name = state.xpr().frame().group().unwrap().0.to_owned();
                                    let mut im = ImString::with_capacity(100);
                                    im.push_str(&name);
                                    let _ = ui.push_item_width(100.);
                                    if ui.input_text(&im_str!(""), &mut im).auto_select_all(true).enter_returns_true(true).build() {
                                        let im: &str = &im.as_ref();
                                        info!("renaming: {}", im);
                                        // state.xpr_mut().rename_layer(&im).unwrap();
                                        state.xpr_mut().frame_mut().group_mut().unwrap().0 = im.to_owned();
                                        state.rename_group = None;
                                        state.toggle_hotkeys();
                                        ui.close_current_popup();
                                    }
                                } else {
                                    if ui.selectable(&im_str!("Rename"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                        info!("renaming layer...");
                                        // disable hotkeys
                                        state.toggle_hotkeys();
                                        state.rename_group = Some(group_id);
                                        ui.close_current_popup();
                                    }
                                    if ui.selectable(&im_str!("Move Up"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                        info!("moving group up...");
                                        if group_id != 0 {
                                            state.xpr_mut().commit();
                                            state.xpr_mut().swap_group(group_id - 1, group_id);
                                            ui.close_current_popup();
                                        }
                                    }
                                    if ui.selectable(&im_str!("Move Down"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                        info!("moving group down...");
                                        if group_id + 1 != ngroups.len() {
                                            state.xpr_mut().commit();
                                            state.xpr_mut().swap_group(group_id, group_id + 1);
                                            ui.close_current_popup();
                                        }
                                    }
                                }
                            });

                            for layer_id in 0..n_layers {
                                ui.with_id(layer_id as i32, || {
                                    let l = group!().1[layer_id].clone();
                                    let mut layer = l.borrow_mut();

                                    if layer_id >= group!().1.len() {
                                        return;
                                    }
                                    if ui.checkbox(&im_str!(""), &mut layer.visible) {
                                        // undo imgui checkbox mutation
                                        layer.visible = !layer.visible;
                                        // enter history frame and toggle
                                        state.xpr_mut().toggle_layer_visibility(group_id, layer_id).unwrap();
                                    }
                                    ui.same_line(0.);

                                    let is_sel = {
                                        let frame = state.xpr().frame();
                                        frame.selected == layer_id && frame.sel_group == group_id
                                    };

                                    if layer_id >= group!().1.len() {
                                        return;
                                    }
                                    let name = layer.name.as_str();
                                    if ui.selectable(&im_str!("{}", name), is_sel, ImGuiSelectableFlags::empty(), [100., 0.]) {
                                        state.xpr_mut().switch_layer(group_id, layer_id);
                                    }

                                    if (ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Right))
                                        || (state.rename_layer.is_some() && state.rename_layer.unwrap() == (group_id, layer_id))
                                    {
                                        state.xpr_mut().switch_layer(group_id, layer_id);
                                        ui.open_popup(&im_str!("contextmenu_layer"));
                                    }

                                    ui.popup(&im_str!("contextmenu_layer"), || {
                                        if state.rename_layer.is_some() {
                                            let l = state.xpr_mut().cel().unwrap();
                                            let name = l.borrow().name.to_owned();
                                            let mut im = ImString::with_capacity(100);
                                            im.push_str(&name);
                                            let _ = ui.push_item_width(100.);
                                            if ui.input_text(&im_str!(""), &mut im).auto_select_all(true).enter_returns_true(true).build() {
                                                let im: &str = &im.as_ref();
                                                info!("renaming: {}", im);
                                                state.xpr_mut().rename_layer(&im).unwrap();
                                                state.rename_layer = None;
                                                state.toggle_hotkeys();
                                                ui.close_current_popup();
                                            }
                                        } else {
                                            if ui.selectable(&im_str!("Rename"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                info!("renaming layer...");
                                                // disable hotkeys
                                                state.toggle_hotkeys();
                                                state.rename_layer = Some((group_id, layer_id));
                                                // ui.close_current_popup();
                                            }

                                            if ui.selectable(&im_str!("Delete"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                state.xpr_mut().remove_layer(group_id, layer_id).unwrap();
                                            }

                                            if ui.selectable(&im_str!("Insert Below"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                state.xpr_mut().commit();
                                                state.xpr_mut().frame_mut().insert_layer(None, true, layer_id + 1);
                                                ui.close_current_popup();
                                            }

                                            if ui.selectable(&im_str!("Insert Above"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                state.xpr_mut().commit();
                                                state.xpr_mut().frame_mut().insert_layer(None, true, layer_id);
                                                ui.close_current_popup();
                                            }

                                            if ui.selectable(&im_str!("Move Up"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                info!("moving layer up...");
                                                if layer_id != 0 {
                                                    state.xpr_mut().commit();
                                                    state.xpr_mut().swap_layer(layer_id - 1, layer_id);
                                                    ui.close_current_popup();
                                                }
                                            }

                                            if ui.selectable(&im_str!("Move Down"), false, ImGuiSelectableFlags::empty(), [0., 0.]) {
                                                info!("moving layer down...");
                                                if layer_id + 1 != n_layers {
                                                    state.xpr_mut().commit();
                                                    state.xpr_mut().swap_layer(layer_id, layer_id + 1);
                                                    ui.close_current_popup();
                                                }
                                            }
                                        }
                                    });
                                });
                            }
                        });
                });
            }
        })
}
