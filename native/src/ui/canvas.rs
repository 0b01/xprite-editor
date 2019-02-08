use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use imgui::ImGuiWindowFlags;
use xprite::rendering::Renderer;

pub fn draw_canvas(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("canvas"))
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
                        state.xpr.render_canvas(rdr);
                        super::inputs::bind_input(state, ui);
                        let origin = state.xpr.canvas.origin();
                        ui.set_cursor_screen_pos([origin.x as f32, origin.y as f32]);
                        rdr.render();

                        state.redraw_pixels(rdr).unwrap();

                        ui.image(
                            ImTexture::from(state.texture.unwrap()),
                            [
                                (state.xpr.canvas.art_w * state.xpr.canvas.scale) as f32,
                                (state.xpr.canvas.art_h * state.xpr.canvas.scale) as f32,
                            ],
                        )
                        .build();
                        draw_cursor_cross(ui);

                        state.xpr.render_cursor(rdr);
                        state.xpr.render_bezier(rdr);
                        state.xpr.render_marquee(rdr);
                    });
            });

            // ui.drag_float(im_str!("scale"), &mut state.xpr.canvas.scale)
            //   .min(1.)
            //   .max(50.)
            //   .speed(0.1)
            //   .build();

            // checkbox for show grid
            ui.checkbox(im_str!("grid"), &mut state.xpr.canvas.show_grid);
            ui.text(im_str!(
                "{}, {}",
                state.xpr.last_mouse_pos.y,
                state.xpr.last_mouse_pos.x
            ));
        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let cvs = &mut state.xpr.canvas;
    let win_pos = ui.get_cursor_screen_pos();
    cvs.update_pos(win_pos.0.into(), win_pos.1.into());

    let canvas_sz = ui.get_window_size();
    cvs.update_sz(canvas_sz.0.into(), canvas_sz.1.into());

    if !cvs.initialized {
        cvs.scale = cvs.canvas_w / cvs.art_w / CANVAS_INIT_SCALE;
        cvs.scroll.x = (cvs.canvas_w - cvs.scale * cvs.art_w) / 2.;
        cvs.scroll.y = (cvs.canvas_h - cvs.scale * cvs.art_h) / 2.;
    }

    state.xpr.canvas.initialized = true;
}

fn draw_cursor_cross(ui: &Ui) {
    let draw_list = ui.get_window_draw_list();
    let (x, y) = ui.imgui().mouse_pos();

    let origin = [x, y];
    let down = [x, y + 10.];
    let up = [x, y - 10.];
    let right = [x + 10., y];
    let left = [x - 10., y];

    let color: [f32; 4] = Color::black().into();
    draw_list.add_line(origin, up, color).build();
    draw_list.add_line(origin, down, color).build();
    draw_list.add_line(origin, left, color).build();
    draw_list.add_line(origin, right, color).build();
}
