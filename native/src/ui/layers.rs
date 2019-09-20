/// TODO: this file is hideous
use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    Window::new(&im_str!("Layers"))
        .bring_to_front_on_focus(false)
        .position([sz[0] as f32 - RIGHT_SIDE_WIDTH, sz[1] as f32 * 2. / 4. + 20.], Condition::Always)
        .size([RIGHT_SIDE_WIDTH, (sz[1] / 4.) as f32 - 20.], Condition::Always)
        .movable(false)
        .collapsible(false)
        .resizable(false)
        .build(&ui, || {
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
                let pushed_id = ui.push_id(group_id as i32);
                    macro_rules! group {
                        () => {
                            state.xpr_mut().frame_mut().groups[group_id]
                        };
                    };

                    ui.tree_node(&im_str!("{}", &group!().0))
                        .default_open(true)
                        .open_on_double_click(false)
                        .build(||{
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
                                    if Selectable::new(&im_str!("Rename"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                        info!("renaming layer...");
                                        // disable hotkeys
                                        state.toggle_hotkeys();
                                        state.rename_group = Some(group_id);
                                        ui.close_current_popup();
                                    }
                                    if Selectable::new(&im_str!("Move Up"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                        info!("moving group up...");
                                        if group_id != 0 {
                                            state.xpr_mut().commit();
                                            state.xpr_mut().swap_group(group_id - 1, group_id);
                                            ui.close_current_popup();
                                        }
                                    }
                                    if Selectable::new(&im_str!("Move Down"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
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
                                let pushed_id = ui.push_id(layer_id as i32);
                                    let l = group!().1[layer_id].clone();

                                    if layer_id >= group!().1.len() {
                                        return;
                                    }

                                    {
                                        let mut layer = l.borrow_mut();
                                        if ui.checkbox(&im_str!(""), &mut layer.visible) {
                                            // undo imgui checkbox mutation
                                            layer.visible = !layer.visible;
                                            drop(layer);
                                            // enter history frame and toggle
                                            state.xpr_mut().toggle_layer_visibility(group_id, layer_id).unwrap();
                                        }
                                        ui.same_line(0.);
                                    }

                                    let is_sel = {
                                        let frame = state.xpr().frame();
                                        frame.layer_idx == layer_id && frame.group_idx == group_id
                                    };

                                    if layer_id >= group!().1.len() {
                                        return;
                                    }
                                    let layer = l.borrow_mut();
                                    let name = layer.name.as_str();
                                    if Selectable::new(&im_str!("{}", name))
                .selected(is_sel)
                .flags(SelectableFlags::empty())
                .size([100., 0.])
                .build(&ui) {
                                        state.xpr_mut().switch_layer(group_id, layer_id);
                                    }

                                    if (ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Right))
                                        || (state.rename_layer.is_some() && state.rename_layer.unwrap() == (group_id, layer_id))
                                    {
                                        state.xpr_mut().switch_layer(group_id, layer_id);
                                        ui.open_popup(&im_str!("contextmenu_layer"));
                                    }

                                    drop(layer);
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
                                            if Selectable::new(&im_str!("Rename"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                info!("renaming layer...");
                                                // disable hotkeys
                                                state.toggle_hotkeys();
                                                state.rename_layer = Some((group_id, layer_id));
                                                // ui.close_current_popup();
                                            }

                                            if Selectable::new(&im_str!("Delete"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                state.xpr_mut().remove_layer(group_id, layer_id).unwrap();
                                            }

                                            if Selectable::new(&im_str!("Insert Below"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                state.xpr_mut().commit();
                                                state.xpr_mut().frame_mut().insert_layer(None, true, layer_id + 1);
                                                ui.close_current_popup();
                                            }

                                            if Selectable::new(&im_str!("Insert Above"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                state.xpr_mut().commit();
                                                state.xpr_mut().frame_mut().insert_layer(None, true, layer_id);
                                                ui.close_current_popup();
                                            }

                                            if Selectable::new(&im_str!("Move Up"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                info!("moving layer up...");
                                                if layer_id != 0 {
                                                    state.xpr_mut().commit();
                                                    state.xpr_mut().swap_layer(layer_id - 1, layer_id);
                                                    ui.close_current_popup();
                                                }
                                            }

                                            if Selectable::new(&im_str!("Move Down"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui) {
                                                info!("moving layer down...");
                                                if layer_id + 1 != n_layers {
                                                    state.xpr_mut().commit();
                                                    state.xpr_mut().swap_layer(layer_id, layer_id + 1);
                                                    ui.close_current_popup();
                                                }
                                            }
                                        }
                                    });
                                pushed_id.pop(&ui);
                            }
                        });
                pushed_id.pop(&ui);
            }
        })
}
