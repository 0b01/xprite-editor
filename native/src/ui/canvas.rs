use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use imgui::ImGuiWindowFlags;
use xprite::rendering::Renderer;

pub fn draw_canvas(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("canvas"))
        .no_bring_to_front_on_focus(true)
        .position((LEFT_SIDE_WIDTH, 20.0), ImGuiCond::Always)
        .size(
            (
                sz.0 as f32 - RIGHT_SIDE_WIDTH - LEFT_SIDE_WIDTH,
                sz.1 as f32 - 20.,
            ),
            ImGuiCond::Always,
        )
        .flags(
            ImGuiWindowFlags::NoBringToFrontOnFocus
                | ImGuiWindowFlags::NoTitleBar
                | ImGuiWindowFlags::NoResize
                | ImGuiWindowFlags::NoMove
                | ImGuiWindowFlags::NoCollapse,
        )
        .build(|| {
            let styles = [
                StyleVar::FramePadding(ImVec2::new(-1., -1.)),
                StyleVar::WindowPadding(ImVec2::new(-1., -1.)),
            ];
            let colors = [(ImGuiCol::ChildBg, BACKGROUND)];

            // if switched, set redraw dirty flg for the new xpr doc
            let mut redraw_idx = None;
            let mut close_idx = None;
            for (i, x) in state.xprs.iter_mut().enumerate() {
                let is_sel = i == state.xpr_idx;
                // let col: [f32; 4] = if is_sel {Color::grey()} else {Color::black()}.into();
                ui.push_id(i as i32);
                ui.same_line(0.);
                if ui.radio_button_bool(im_str!("{}", x.name), is_sel) {
                    state.xpr_idx = i;
                    redraw_idx = Some(i);
                }
                // right click
                if ui.is_item_hovered() && ui.imgui().is_mouse_clicked(ImMouseButton::Right)
                {
                    info!("right clicked");
                    ui.open_popup(im_str!("contextmenu_doc##{}", i));
                }
                ui.popup(im_str!("contextmenu_doc##{}", i), || {
                    if ui.selectable( im_str!("Close"), false, ImGuiSelectableFlags::empty(), (50., 0.),) {
                        info!("close pressed");
                        close_idx = Some(i);
                        ui.close_current_popup();
                    }
                });

                ui.pop_id();

            }

            if let Some(cidx) = close_idx {
                state.execute(Bind::CloseXpr(cidx));
                return;
            }

            if let Some(ridx) = redraw_idx {
                state.xprs[ridx].redraw = true;
            }

            ui.with_style_and_color_vars(&styles, &colors, || {
                let win_sz = ui.get_window_size();
                let child_frame_sz = (win_sz.0, win_sz.1);
                ui.child_frame(im_str!("scrolling_region"), child_frame_sz)
                    .show_scrollbar(false)
                    .movable(false)
                    .build(|| {
                        // diable cursor
                        if ui.is_window_hovered() {
                            ui.imgui().set_mouse_cursor(ImGuiMouseCursor::None);
                        }

                        update_viewport(state, ui);
                        super::inputs::bind_input(state, ui);
                        let origin = state.xpr_mut().canvas.origin();
                        ui.set_cursor_screen_pos([
                            origin.x as f32,
                            origin.y as f32,
                        ]);
                        rdr.render();

                        state.redraw_pixels(rdr).unwrap();

                        ui.image(
                            ImTexture::from(state.texture.unwrap()),
                            [
                                (state.xpr_mut().canvas.art_w
                                    * state.xpr_mut().canvas.scale)
                                    as f32,
                                (state.xpr_mut().canvas.art_h
                                    * state.xpr_mut().canvas.scale)
                                    as f32,
                            ],
                        )
                        .build();

                        state.xpr_mut().render(rdr);

                        if state.xpr_mut().toolbox.selected == ToolType::ColorPicker {
                            draw_color_picker(state, ui);
                        } else {
                            draw_cursor_cross(ui);
                        }
                    });
            });

            // ui.drag_float(im_str!("scale"), &mut state.xpr_mut().canvas.scale)
            //   .min(1.)
            //   .max(50.)
            //   .speed(0.1)
            //   .build();

            // checkbox for show grid
            ui.checkbox(im_str!("grid"), &mut state.xpr_mut().canvas.show_grid);
            ui.text(im_str!(
                "{}, {}",
                state.xpr().last_mouse_pos.y,
                state.xpr().last_mouse_pos.x
            ));
        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let cvs = &mut state.xpr_mut().canvas;
    let win_pos = ui.get_cursor_screen_pos();
    cvs.update_pos(win_pos.0.into(), win_pos.1.into());

    let canvas_sz = ui.get_window_size();
    cvs.update_sz(canvas_sz.0.into(), canvas_sz.1.into());

    if !cvs.initialized {
        cvs.scale = cvs.canvas_w / cvs.art_w / CANVAS_INIT_SCALE;
        cvs.scroll.x = (cvs.canvas_w - cvs.scale * cvs.art_w) / 2.;
        cvs.scroll.y = (cvs.canvas_h - cvs.scale * cvs.art_h) / 2.;
    }

    state.xpr_mut().canvas.initialized = true;
}

fn draw_cursor_cross(ui: &Ui) {
    let draw_list = ui.get_window_draw_list();
    let (x, y) = ui.imgui().mouse_pos();

    let l = 5.;
    let origin = [x, y];

    let down1 = [x, y + l];
    let up1 = [x, y - l];
    let right1 = [x + l, y];
    let left1 = [x - l, y];

    let down2 = [x, y + 2. * l];
    let up2 = [x, y - 2. * l];
    let right2 = [x + 2. * l, y];
    let left2 = [x - 2. * l, y];

    let color1: [f32; 4] = Color::black().into();
    let color2: [f32; 4] = Color::white().into();

    draw_list.add_line(origin, up1, color1).build();
    draw_list.add_line(origin, down1, color1).build();
    draw_list.add_line(origin, left1, color1).build();
    draw_list.add_line(origin, right1, color1).build();

    draw_list.add_line(up1, up2, color2).build();
    draw_list.add_line(down1, down2, color2).build();
    draw_list.add_line(left1, left2, color2).build();
    draw_list.add_line(right1, right2, color2).build();
}


fn draw_color_picker(state: &mut State, ui: &Ui) {
    let (x, y) = ui.imgui().mouse_pos();
    ui.set_cursor_screen_pos([x-10.,y-10.]);
    ui.image(
        ImTexture::from(state.color_picker_texture.unwrap()),
        [20., 20.],
    )
    .build();
}