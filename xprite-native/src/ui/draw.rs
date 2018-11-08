use crate::state::State;
use imgui::*;
use xprite::prelude::*;
use xprite::rendering::Renderer;


/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &Renderer, state: &mut State, ui: &Ui) -> bool {
    main_menu_bar(rdr, state, ui);
    draw_canvas(rdr, state, ui);
    true
}

fn main_menu_bar(_rdr: &Renderer, _state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            ui.menu_item(im_str!("Load")).shortcut(im_str!("Ctrl+O")).build();
            ui.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build();
        })
    })
}

fn draw_canvas(rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("canvas"))
        .position((20.0, 20.0), ImGuiCond::Appearing)
        // .size((700.0, 300.0), ImGuiCond::Appearing)
        .size((sz.0 as f32/2., sz.1 as f32/2.), ImGuiCond::Appearing)
        .resizable(true)
        .build(|| {
            // checkbox for show grid
            ui.checkbox(im_str!("grid"), &mut state.xpr.canvas.show_grid);
            ui.drag_float(im_str!("scale"), &mut state.xpr.canvas.scale)
              .min(1.)
              .max(50.)
              .speed(0.1)
              .build();

            let styles = [
                StyleVar::FramePadding(ImVec2::new(1., 1.)),
                StyleVar::WindowPadding(ImVec2::new(0., 0.)),
            ];
            let colors = [ (ImGuiCol::ChildBg, GREY) ];

            ui.with_style_and_color_vars(&styles, &colors, || {
                ui.child_frame(im_str!("scrolling_region"), (0., 0.,))
                    .show_scrollbar(false)
                    .movable(false)
                    .build(|| {
                        update_viewport(state, ui);
                        state.xpr.render(rdr);
                        // draw_grid(state, ui);
                        bind_input(state, ui);

                    });
            });

        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let win_pos = ui.get_cursor_screen_pos();
    state.xpr.canvas.update_pos(win_pos.0, win_pos.1);

    let canvas_sz = ui.get_window_size();
    state.xpr.canvas.update_sz(canvas_sz.0, canvas_sz.1);
}

fn bind_input(state: &mut State, ui: &Ui) {
    use self::MouseButton::*;
    use self::MouseEvent::*;

    let wheel_delta = ui.imgui().mouse_wheel();
    let (x, y) = ui.imgui().mouse_pos();

    if state.last_mouse_pos.0 != x
    || state.last_mouse_pos.1 != y {
        state.xpr.mouse_move(&MouseMove{ x, y });
    }

    let left_mouse_down = ui.imgui().is_mouse_down(ImMouseButton::Left);

    // middle key for scrolling
    if ui.is_window_hovered() && !ui.is_item_active() &&
        ui.imgui().is_mouse_dragging(ImMouseButton::Middle)
    {
        let d = ui.imgui().mouse_delta();
        state.xpr.canvas.scroll.x += d.0;
        state.xpr.canvas.scroll.y += d.1;
    }

    if ui.is_window_hovered() && !ui.is_item_active()
    {
        state.xpr.canvas.scale += wheel_delta;
    }

    // left up
    if state.is_left_mouse_down && !left_mouse_down {
        state.xpr.mouse_up(&MouseUp{ x, y });
        state.is_left_mouse_down = false;
    }

    // left down
    if ui.is_window_hovered() && !ui.is_item_active() &&
        left_mouse_down && !state.is_left_mouse_down
    {
        println!("mouse left down");
        state.xpr.mouse_down(&MouseDown{ x, y, button: Left });
        state.is_left_mouse_down = true;
    }

    //right down
    if ui.is_window_hovered() && !ui.is_item_active() &&
        ui.imgui().is_mouse_down(ImMouseButton::Right)
    {
        let (x, y) = ui.imgui().mouse_pos();
        state.xpr.mouse_down(&MouseDown{ x, y, button: Right });
    }

    state.update_mouse_pos(x, y);
}