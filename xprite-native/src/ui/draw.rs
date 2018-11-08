use crate::state::State;
use imgui::*;
use xprite::prelude::*;
use xprite::rendering::Renderer;


const WHITE: [f32;4] = [256.,256.,256.,256.];
const RED: [f32;4] = [256.,0.,0.,256.];
const GREY: [f32;4] = [10.,10.,10.,256.];
const BLACK: [f32;4] = [0.,0.,0.,0.];

/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &Renderer, state: &mut State, ui: &Ui) -> bool {
    main_menu_bar(rdr, state, ui);
    draw_canvas(rdr, state, ui);
    true
}

fn main_menu_bar(rdr: &Renderer, _state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            ui.menu_item(im_str!("Load")).shortcut(im_str!("Ctrl+O")).build();
            ui.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build();
        })
    })
}

fn draw_canvas(rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui.window(im_str!("canvas"))
        .position((20.0, 20.0), ImGuiCond::Appearing)
        .size((700.0, 300.0), ImGuiCond::Appearing)
        .resizable(true)
        .build(|| {
            // checkbox for show grid
            ui.checkbox(im_str!("Show grid"), &mut state.xpr.canvas.show_grid);
            let styles = [
                StyleVar::FramePadding(ImVec2::new(1., 1.)),
                StyleVar::WindowPadding(ImVec2::new(0., 0.)),
            ];
            let colors = [ (ImGuiCol::ChildBg, RED) ];

            ui.with_style_and_color_vars(&styles, &colors, || {
                ui.child_frame(im_str!("scrolling_region"), (0., 0.,))
                    .show_scrollbar(false)
                    .movable(false)
                    .build(|| {
                        update_dims(state, ui);
                        state.xpr.render(rdr);
                        // draw_grid(state, ui);

                        if ui.is_window_hovered() && !ui.is_item_active() && ui.imgui().is_mouse_dragging(ImMouseButton::Middle) {
                            let d = ui.imgui().mouse_delta();
                            state.xpr.canvas.view.x0 += d.0;
                            state.xpr.canvas.view.y0 += d.1;
                      }
                    });
            });

        });
}

fn update_dims(state: &mut State, ui: &Ui) {
    let win_pos = ui.get_cursor_screen_pos();
    state.xpr.canvas.update_pos(win_pos.0, win_pos.1);

    let canvas_sz = ui.get_window_size();
    state.xpr.canvas.update_sz(canvas_sz.0, canvas_sz.1);
}

fn draw_grid(state: &mut State, ui: &Ui) {
    if !state.xpr.canvas.show_grid { return }
    let cvs = &mut state.xpr.canvas;

    let draw_list = ui.get_window_draw_list();
    let color = WHITE;
    let sz = 64.;
    let win_pos = ui.get_cursor_screen_pos();
    let canvas_sz = ui.get_window_size();
    let mut x = cvs.view.x0 % sz;
    while x < canvas_sz.0 {
        draw_list.add_line(
            (x + win_pos.0, 0. + win_pos.1),
            (x + win_pos.0, canvas_sz.1 + win_pos.1),
            color
        ).build();
        x += sz;
    }
    let mut y = cvs.view.y0 % sz;
    while y < canvas_sz.1 {
        draw_list.add_line(
            (0. + win_pos.0, y + win_pos.1),
            (canvas_sz.0 + win_pos.0, y + win_pos.1),
            color
        ).build();
        y += sz;
    }
}