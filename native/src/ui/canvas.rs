use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use imgui::WindowFlags;
use xprite::rendering::Renderer;

pub fn draw_canvas(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) {
    let sz = ui.io().display_size;
    Window::new(&im_str!("canvas"))
        .bring_to_front_on_focus(false)
        .position([LEFT_SIDE_WIDTH, 20.0], Condition::Always)
        .size([sz[0] as f32 - RIGHT_SIDE_WIDTH - LEFT_SIDE_WIDTH, sz[1] as f32 - 20.], Condition::Always)
        .flags(
            WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS
                | WindowFlags::NO_TITLE_BAR
                | WindowFlags::NO_RESIZE
                | WindowFlags::NO_MOVE
                | WindowFlags::NO_COLLAPSE,
        )
        .build(&ui, || {
            let styles = [StyleVar::FramePadding([-1., -1.]), StyleVar::WindowPadding([-1., -1.])];
            let colors = [(StyleColor::ChildBg, BACKGROUND)];

            // if switched, set redraw dirty flg for the new xpr doc
            let mut redraw_idx = None;
            let mut close_idx = None;
            for (i, x) in state.xprs.iter_mut().enumerate() {
                let is_sel = i == state.xpr_idx;
                // let col: [f32; 4] = if is_sel {Color::grey()} else {Color::black()}.into();
                let pushed_id = ui.push_id(i as i32);
                ui.same_line(0.);
                if ui.radio_button_bool(&im_str!("{}", x.name), is_sel) {
                    state.xpr_idx = i;
                    redraw_idx = Some(i);
                }
                // right click
                if ui.is_item_hovered() && ui.is_mouse_clicked(MouseButton::Right) {
                    info!("right clicked");
                    ui.open_popup(&im_str!("contextmenu_doc##{}", i));
                }
                ui.popup(&im_str!("contextmenu_doc##{}", i), || {
                    if Selectable::new(&im_str!("Close"))
                .selected(false)
                .flags(SelectableFlags::empty())
                .size([50., 0.])
                .build(&ui) {
                        info!("close pressed");
                        close_idx = Some(i);
                        ui.close_current_popup();
                    }
                });

                pushed_id.pop(&ui);
            }

            if let Some(cidx) = close_idx {
                state.execute(Bind::CloseXpr(cidx)).unwrap();
                return;
            }

            if let Some(ridx) = redraw_idx {
                state.xprs[ridx].set_redraw(true);
            }

            let style = ui.push_style_vars(&styles);
            let color = ui.push_style_colors(&colors);
            ChildWindow::new(&im_str!("scrolling_region"))
                .size(ui.io().display_size)
                .scroll_bar(false)
                // .movable(false)
                .build(&ui, || {
                    // diable cursor
                    if ui.is_window_hovered() {
                        ui.set_mouse_cursor(None);
                    }

                    update_viewport(state, ui);
                    super::inputs::bind_input(state, ui);

                    let origin = state.xpr_mut().canvas.origin();
                    ui.set_cursor_screen_pos([origin.x as f32, origin.y as f32]);
                    rdr.render(Some(state.xpr())).unwrap();

                    state.redraw_pixels(rdr).unwrap();

                    Image::new(TextureId::from(state.texture.unwrap()), [
                            (state.xpr_mut().canvas.art_w * state.xpr_mut().canvas.scale) as f32,
                            (state.xpr_mut().canvas.art_h * state.xpr_mut().canvas.scale) as f32,
                        ]).build(&ui);

                    state.xpr_mut().render(rdr);

                    if state.xpr_mut().toolbox.selected == ToolType::ColorPicker {
                        draw_color_picker_icon(state, ui);
                    } else {
                        draw_cursor_cross(ui);
                    }
                });

            // ui.drag_float(&im_str!("scale"), &mut state.xpr_mut().canvas.scale)
            //   .min(1.)
            //   .max(50.)
            //   .speed(0.1)
            //   .build(&ui);
            style.pop(&ui);
            color.pop(&ui);

        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let cvs = &mut state.xpr_mut().canvas;
    let win_pos = ui.cursor_screen_pos();
    cvs.update_pos(win_pos[0].into(), win_pos[1].into());

    let canvas_sz = ui.window_size();
    cvs.update_sz(canvas_sz[0].into(), canvas_sz[1].into());

    if !cvs.initialized {
        cvs.scale = cvs.canvas_w / cvs.art_w / CANVAS_INIT_SCALE;
        // cvs.scroll.x = (cvs.canvas_w - cvs.scale * cvs.art_w) / 2.;
        // cvs.scroll.y = (cvs.canvas_h - cvs.scale * cvs.art_h) / 2.;
        // cvs.scroll.x = 0.;
        // cvs.scroll.y = 0.;
    }

    state.xpr_mut().canvas.initialized = true;
}

fn draw_cursor_cross(ui: &Ui) {
    let draw_list = ui.get_window_draw_list();
    let pos = ui.io().mouse_pos;
    let x = pos[0].into();
    let y = pos[1].into();

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

    let color1: [f32; 4] = XpriteRgba::black().into();
    let color2: [f32; 4] = XpriteRgba::white().into();

    draw_list.add_line(origin, up1, color1).build();
    draw_list.add_line(origin, down1, color1).build();
    draw_list.add_line(origin, left1, color1).build();
    draw_list.add_line(origin, right1, color1).build();

    draw_list.add_line(up1, up2, color2).build();
    draw_list.add_line(down1, down2, color2).build();
    draw_list.add_line(left1, left2, color2).build();
    draw_list.add_line(right1, right2, color2).build();
}

fn draw_color_picker_icon(state: &mut State, ui: &Ui) {
    let pos = ui.io().mouse_pos;
    let x: f32 = pos[0].into();
    let y: f32 = pos[1].into();
    ui.set_cursor_screen_pos([x - 10., y - 10.]);
    Image::new(TextureId::from(state.icons["color_picker"]), [20., 20.]).build(&ui);
}
