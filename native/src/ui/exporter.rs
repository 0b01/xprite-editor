use crate::prelude::*;
use crate::state::exporter_state::ExporterFormat;
use xprite::rendering::Renderer;

pub fn draw_exporter(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    if state.exporter_state.show {
        ui.window(im_str!("Exporter"))
            .size((300., 200.), ImGuiCond::Appearing)
            .movable(true)
            .collapsible(false)
            .resizable(true)
            .build(|| {
                if ui.button(im_str!("+"), (0.,0.)) {
                    state.exporter_state.add_default();
                }
                ui.same_line(0.);
                if ui.button(im_str!("export"), (0.,0.)) {
                    state.export();
                }
                let mut im = ImString::with_capacity(100);
                im.push_str(&state.exporter_state.path);
                if ui.input_text(im_str!("Path"), &mut im).build() {
                    state.exporter_state.path = im.to_str().to_owned();
                }
                let len = state.exporter_state.specs.len();
                'out: for i in 0..len {
                    ui.push_id(i as i32);
                    macro_rules! spec {
                        () => {
                            state.exporter_state.specs[i]
                        }
                    }
                    if ui.button(im_str!("-"), (0.,0.)) {
                        state.exporter_state.remove(i);
                        ui.pop_id();
                        break 'out;
                    }
                    ui.same_line(0.);
                    ui.button(im_str!("{:?}", spec!().format), (0.,0.,));
                    if ui.is_item_hovered()
                    && ui.imgui().is_mouse_down(ImMouseButton::Left) {
                        ui.open_popup(im_str!("File Format"));
                        ui.close_current_popup();
                    }

                    ui.popup(im_str!("File Format"), || {
                        for spec in &ExporterFormat::VARIANTS {
                            if spec == &ExporterFormat::ICO
                                && (state.xpr.canvas.art_w > 255. ||
                                    state.xpr.canvas.art_h > 255.)
                            {
                                ui.button(im_str!("ico"), (0.,0.));
                                if ui.is_item_hovered() {
                                    ui.tooltip_text("artwork too big (limit: 1<w,h<256)")
                                }
                                continue;
                            }
                            if ui.button(im_str!("{:#?}", spec), (0.,0.)) {
                                state.exporter_state.set_format(i, *spec);
                            }
                        }
                    });

                    if spec!().format != ExporterFormat::ASE {
                        let mut scale = spec!().rescale as i32;
                        if ui.drag_int(im_str!("scale"), &mut scale).min(1).max(100).build() {
                            state.exporter_state.set_scale(i, scale as u32);
                        }
                    }

                    let mut fname = ImString::with_capacity(100);
                    fname.push_str(&spec!().stem);
                    if ui.input_text(im_str!("Filename"),  &mut fname).build() {
                        state.exporter_state.set_stem(i, fname.to_str().to_owned());
                    }



                    ui.pop_id();
                }
            });
    }
}