use crate::prelude::*;
use imgui::ImGuiWindowFlags;
use xprite::rendering::Renderer;
use crate::render::imgui::ImguiRenderer;

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
                        update_viewport(state, ui);
                        state.xpr.render(rdr);
                        super::inputs::bind_input(state, ui);
                        let origin = state.xpr.canvas.origin();
                        ui.set_cursor_screen_pos([origin.x, origin.y]);
                        rdr.render();

                        state.redraw_pixels(rdr).unwrap();

                        ui.image(ImTexture::from(state.preview_texture.unwrap()), [
                            rdr.art_w * state.xpr.canvas.scale,
                            rdr.art_h * state.xpr.canvas.scale,
                        ]).build();

                        state.xpr.render_cursor(rdr);
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
    cvs.update_pos(win_pos.0, win_pos.1);

    let canvas_sz = ui.get_window_size();
    cvs.update_sz(canvas_sz.0, canvas_sz.1);

    if !cvs.initialized {
        cvs.scale = cvs.canvas_w / cvs.art_w / CANVAS_INIT_SCALE;
        cvs.scroll.x = (cvs.canvas_w - cvs.scale * cvs.art_w) / 2.;
        cvs.scroll.y = (cvs.canvas_h - cvs.scale * cvs.art_h) / 2.;
    }

    state.xpr.canvas.initialized = true;
}
