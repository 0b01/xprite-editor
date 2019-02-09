use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("Layers"))
        .no_bring_to_front_on_focus(true)
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
            if ui.is_item_hovered() {
                ui.tooltip_text("Add layer under selected group");
            }
            ui.same_line(70.);
            if ui.button(im_str!("+Group"), (60., 20.)) {
                state.xpr.history.top_mut().add_group(None);
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("Add group");
            }

            let ngroups: Vec<_> = {
                let top = state.xpr.history.top_mut();
                if top.selected_layer_mut().is_none() {
                    return;
                }
                top.groups.iter().map(|i| i.1.len()).collect()
            };

            for (group_id, group_len) in ngroups.into_iter().enumerate() {
                ui.with_id(group_id as i32, || {
                    macro_rules! group {
                        () => {
                            state.xpr.history.top_mut().groups[group_id]
                        };
                    };

                    ui.tree_node(im_str!("{}", &group!().0))
                        .default_open(true)
                        .open_on_double_click(false)
                        .build(|| {
                            if (ui.is_item_hovered()
                                && ui.imgui().is_mouse_clicked(ImMouseButton::Right))
                                || (state.rename_group.is_some()
                                    && state.rename_group.unwrap() == group_id)
                            {
                                info!("clicked group");
                                // change selected group
                                state.xpr.switch_layer(group_id, 0);
                                ui.open_popup(im_str!("contextmenu_group##{}", group_id));
                            }
                            ui.popup(im_str!("contextmenu_group##{}", group_id), || {
                                if state.rename_group.is_some() {
                                    let name = state
                                        .xpr
                                        .history
                                        .top()
                                        .selected_group()
                                        .unwrap()
                                        .0
                                        .to_owned();
                                    let mut im = ImString::with_capacity(100);
                                    im.push_str(&name);
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
                                            state
                                                .xpr
                                                .history
                                                .top_mut()
                                                .selected_group_mut()
                                                .unwrap()
                                                .0 = im.to_owned();
                                            state.rename_group = None;
                                            state.toggle_hotkeys();
                                            ui.close_current_popup();
                                        }
                                    });
                                } else {
                                    if ui.selectable(
                                        im_str!("Rename"),
                                        false,
                                        ImGuiSelectableFlags::empty(),
                                        (50., 0.),
                                    ) {
                                        info!("renaming layer...");
                                        // disable hotkeys
                                        state.toggle_hotkeys();
                                        state.rename_group = Some(group_id);
                                        ui.close_current_popup();
                                    }
                                }
                            });
                            // if state.rename_group.is_some() && state.rename_group.unwrap() == group_id {
                            //     ui.open_popup(im_str!("Rename Group"));
                            // }

                            for i in 0..group_len {
                                ui.with_id(i as i32, || {
                                    macro_rules! layer {
                                        () => {
                                            group!().1[i]
                                        };
                                    }

                                    if i >= group!().1.len() {
                                        return;
                                    }
                                    if ui.checkbox(im_str!(""), &mut layer!().visible) {
                                        // undo imgui checkbox mutation
                                        layer!().visible = !layer!().visible;
                                        // enter history frame and toggle
                                        state.xpr.toggle_layer_visibility(group_id, i).unwrap();
                                    }
                                    ui.same_line(60.);
                                    if ui.button(im_str!("X"), (20., 20.)) {
                                        state.xpr.remove_layer(group_id, i).unwrap();
                                    }
                                    ui.same_line(90.);

                                    let is_sel = {
                                        let top = state.xpr.history.top();
                                        top.selected == i && top.sel_group == group_id
                                    };

                                    if i >= group!().1.len() {
                                        return;
                                    }
                                    let name = layer!().name.as_str();
                                    if ui.selectable(
                                        im_str!("{}", name),
                                        is_sel,
                                        ImGuiSelectableFlags::empty(),
                                        (100., 0.),
                                    ) {
                                        state.xpr.switch_layer(group_id, i);
                                    }

                                    if (ui.is_item_hovered()
                                        && ui.imgui().is_mouse_clicked(ImMouseButton::Right))
                                        || (state.rename_layer.is_some()
                                            && state.rename_layer.unwrap() == (group_id, i))
                                    {
                                        state.xpr.switch_layer(group_id, i);
                                        ui.open_popup(im_str!("contextmenu_layer"));
                                    }

                                    ui.popup(im_str!("contextmenu_layer"), || {
                                        if state.rename_layer.is_some() {
                                            let name = {
                                                state.xpr.current_layer().unwrap().name.to_owned()
                                            };
                                            let mut im = ImString::with_capacity(100);
                                            im.push_str(&name);
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
                                                    state.rename_layer = None;
                                                    state.toggle_hotkeys();
                                                    ui.close_current_popup();
                                                }
                                            });
                                        } else {
                                            if ui.selectable(
                                                im_str!("Rename"),
                                                false,
                                                ImGuiSelectableFlags::empty(),
                                                (50., 0.),
                                            ) {
                                                info!("renaming layer...");
                                                // disable hotkeys
                                                state.toggle_hotkeys();
                                                state.rename_layer = Some((group_id, i));
                                                // ui.close_current_popup();
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
