use crate::prelude::*;
use xprite::core::exporter::{ExportType, ExporterFormat};
use xprite::rendering::Renderer;

pub fn draw_exporter(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    if state.exporter.show {
        ui.window(&im_str!("Exporter"))
            .size([300., 200.], Condition::Appearing)
            .movable(true)
            .collapsible(false)
            .resizable(true)
            .build(|| {
                let mut im = ImString::with_capacity(100);
                im.push_str(&state.exporter.path);
                if ui.input_text(&im_str!("Path"), &mut im).build() {
                    state.exporter.path = im.to_str().to_owned();
                }

                ui.same_line(0.);
                if ui.button(&im_str!("Browse"), [0., 0.]) {
                    let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
                        panic!(e);
                    });
                    match result {
                        nfd::Response::Okay(dir_name) => {
                            state.exporter.path = dir_name;
                        }
                        nfd::Response::OkayMultiple(files) => println!("Files {:?}", files),
                        nfd::Response::Cancel => println!("User canceled"),
                    };
                }

                let len = state.exporter.specs.len();
                'out: for i in 0..len {
                    ui.push_id(i as i32);
                    macro_rules! spec {
                        () => {
                            state.exporter.specs[i]
                        };
                    }
                    if ui.button(&im_str!("-"), [0., 0.]) {
                        state.exporter.remove(i);
                        ui.pop_id();
                        break 'out;
                    }
                    ui.same_line(0.);
                    ui.button(&im_str!("{:?}", spec!().format), [0., 0.]);
                    if ui.is_item_hovered() && ui.is_mouse_down(MouseButton::Left) {
                        ui.open_popup(&im_str!("File Format"));
                        ui.close_current_popup();
                    }

                    ui.same_line(0.);
                    ui.checkbox(&im_str!("trim"), &mut spec!().trim);

                    ui.text("layer(s):");
                    ui.same_line(0.);
                    if ui.radio_button_bool(&im_str!("all"), spec!().layer == ExportType::All) {
                        spec!().layer = ExportType::All;
                    }
                    ui.same_line(0.);
                    if ui.radio_button_bool(&im_str!("one"), if let ExportType::Layer(..) = spec!().layer { true } else { false }) {
                        spec!().layer = ExportType::Layer(0, 0);
                    }
                    ui.same_line(0.);
                    if ui.radio_button_bool(&im_str!("group"), if let ExportType::Group(..) = spec!().layer { true } else { false }) {
                        spec!().layer = ExportType::Group(0);
                    }
                    match spec!().layer {
                        ExportType::All => (),
                        ExportType::Layer(group_id, layer_id) => {
                            ui.same_line(0.);
                            let l = state.xpr().get_layer(group_id as usize, layer_id as usize);
                            let sel_layer_name = &l.borrow().name;
                            if ui.button(&im_str!("{}", sel_layer_name), [0., 0.]) {
                                ui.open_popup(&im_str!("select_export_layer"));
                            }
                            ui.popup(&im_str!("select_export_layer"), || {
                                let mut to_change = None;
                                for (g_id, (name, g)) in state.xpr_mut().frame_mut().groups.iter().enumerate() {
                                    ui.push_id(g_id as i32);
                                    ui.tree_node(&im_str!("{}", name)).default_open(true).build(|| {
                                        for (l_id, layer) in g.iter().enumerate() {
                                            let layer = layer.borrow();
                                            ui.push_id(l_id as i32);
                                            if ui.selectable(&im_str!("{}", layer.name), false, ImGuiSelectableFlags::empty(), [50., 0.]) {
                                                to_change = Some((ExportType::Layer(g_id, l_id), layer.name.clone()));
                                                ui.close_current_popup();
                                            }
                                            ui.pop_id();
                                        }
                                    });
                                    ui.pop_id();
                                }

                                if let Some((to_change, stem)) = to_change {
                                    spec!().layer = to_change;
                                    spec!().stem = stem;
                                }
                            });
                        }
                        ExportType::Group(group_id) => {
                            ui.same_line(0.);
                            let name = &state.xpr_mut().frame_mut().groups[group_id as usize].0;
                            if ui.button(&im_str!("{}", name), [0., 0.]) {
                                ui.open_popup(&im_str!("select_export_layer"));
                            }
                            ui.popup(&im_str!("select_export_layer"), || {
                                let mut to_change = None;
                                for (g_id, (name, _g)) in state.xpr_mut().frame_mut().groups.iter().enumerate() {
                                    ui.push_id(g_id as i32);
                                    if ui.selectable(&im_str!("{}", name), false, ImGuiSelectableFlags::empty(), [50., 0.]) {
                                        to_change = Some(ExportType::Group(g_id));
                                        ui.close_current_popup();
                                    }
                                    ui.pop_id();
                                }

                                if let Some(to_change) = to_change {
                                    spec!().layer = to_change;
                                }
                            });
                        }
                    }

                    ui.popup(&im_str!("File Format"), || {
                        for spec in &ExporterFormat::VARIANTS {
                            if spec == &ExporterFormat::ICO && (state.xpr_mut().canvas.art_w > 255. || state.xpr_mut().canvas.art_h > 255.) {
                                ui.button(&im_str!("ico"), [0., 0.]);
                                if ui.is_item_hovered() {
                                    ui.tooltip_text("artwork too big (limit: 1<w,h<256)")
                                }
                                continue;
                            }
                            if ui.button(&im_str!("{:#?}", spec), [0., 0.]) {
                                state.exporter.set_format(i, *spec);
                                ui.close_current_popup();
                            }
                        }
                    });

                    if spec!().format != ExporterFormat::ASE {
                        let mut scale = spec!().rescale as i32;
                        if ui.drag_int(&im_str!("scale"), &mut scale).min(1).max(100).build() {
                            state.exporter.set_scale(i, scale as u32);
                        }
                    }

                    let mut fname = ImString::with_capacity(100);
                    fname.push_str(&spec!().stem);
                    if ui.input_text(&im_str!("Filename"), &mut fname).build() {
                        state.exporter.set_stem(i, fname.to_str().to_owned());
                    }

                    ui.pop_id();
                }

                if ui.button(&im_str!("+"), [0., 0.]) {
                    state.exporter.add_default(); // TODO:
                }

                if ui.button(&im_str!("Run export"), [0., 0.]) {
                    state.export();
                }

            });
    }
}
