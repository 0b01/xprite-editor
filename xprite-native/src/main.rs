extern crate xprite;
extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use imgui::*;
use xprite::prelude::*;

const WHITE: [f32;4] = [256.,256.,256.,256.];
const BLACK: [f32;4] = [0.,0.,0.,0.];

mod support;

const CLEAR_COLOR: [f32; 4] = BLACK;

struct State {
    show_grid: bool,
    notify_text: &'static str,
    scrolling: imgui::ImVec2,
}

impl State {
    fn reset(&mut self) {
        self.notify_text = "";
    }
}

impl Default for State {
    fn default() -> State {
        State {
            show_grid: true,
            notify_text: "",
            scrolling: imgui::ImVec2::new(0.,0.),
        }
    }
}

fn main() {
    let mut state = State::default();
    support::run("color_button.rs".to_owned(), CLEAR_COLOR, |ui| {
        draw_canvas(&mut state, ui);
        // example_selector(&mut state, ui);
        // match state.show_grid {
        //     1 => example_1(&mut state, ui),
        //     2 => example_2(ui),
        //     _ => (),
        // }
        true
    });
}

fn add(v1: ImVec2, v2: ImVec2) -> ImVec2 {
    ImVec2::new(v1.x + v2.x, v1.y + v2.y)
}

fn draw_canvas(state: &mut State, ui: &Ui) {
    ui.window(im_str!("canvas"))
        .position((20.0, 20.0), ImGuiCond::Appearing)
        .size((700.0, 80.0), ImGuiCond::Appearing)
        .resizable(true)
        .build(|| {
            // checkbox for show grid
            ui.checkbox(im_str!("Show grid"), &mut state.show_grid);
            let styles = [
                StyleVar::FramePadding(ImVec2::new(1., 1.)),
                StyleVar::WindowPadding(ImVec2::new(0., 0.)),
            ];
            let colors = [ (ImGuiCol::ChildBg, (0.,0.,0.,0.,)) ];

            ui.with_style_and_color_vars(&styles, &colors, || {
                ui.child_frame(im_str!("scrolling_region"), (0., 0.,))
                  .show_scrollbar(false)
                  .movable(false)
                  .build(|| {
                    if state.show_grid {
                        let draw_list = ui.get_window_draw_list();
                        let color = WHITE;
                        let sz = 64.;
                        let win_pos = ui.get_cursor_screen_pos().into();
                        let canvas_sz = ui.get_window_size();
                        let mut x = state.scrolling.x % sz;
                        while x < canvas_sz.0 {
                            draw_list.add_line(
                                add(ImVec2::new(x, 0.), win_pos),
                                add(ImVec2::new(x, canvas_sz.1), win_pos),
                                color
                            ).build();
                            x += sz;
                        }
                        let mut y = state.scrolling.y % sz;
                        while y < canvas_sz.1 {
                            draw_list.add_line(
                                add(ImVec2::new(0., y), win_pos),
                                add(ImVec2::new(canvas_sz.0, y), win_pos),
                                color
                            ).build();
                            y += sz;
                        }
                    }

                    if ui.is_window_hovered() && !ui.is_item_active() && ui.imgui().is_mouse_dragging(ImMouseButton::Middle) {
                        let d = ui.imgui().mouse_delta();
                        state.scrolling.x += d.0;
                        state.scrolling.y += d.1;
                    }
                  });
            });

        });
}

// fn example_selector(state: &mut State, ui: &Ui) {
//     ui.window(im_str!("Color button examples"))
//         .position((20.0, 20.0), ImGuiCond::Appearing)
//         .size((700.0, 80.0), ImGuiCond::Appearing)
//         .resizable(false)
//         .build(|| {
//             let ex1 = ui.radio_button(im_str!("Example 1: Basics"), &mut state.example, 1);
//             let ex2 = ui.radio_button(im_str!("Example 2: Alpha component"), &mut state.example, 2);
//             if ex1 || ex2 {
//                 state.reset();
//             }
//         });
// }

// fn example_1(state: &mut State, ui: &Ui) {
//     ui.window(im_str!("Example 1: Basics"))
//         .size((700.0, 300.0), ImGuiCond::Appearing)
//         .position((20.0, 120.0), ImGuiCond::Appearing)
//         .build(|| {
//             ui.text_wrapped(im_str!(
//                 "Color button is a widget that displays a color value as a clickable rectangle. \
//                  It also supports a tooltip with detailed information about the color value. \
//                  Try hovering over and clicking these buttons!"
//             ));
//             ui.text(state.notify_text);

//             ui.text("This button is black:");
//             if ui
//                 .color_button(im_str!("Black color"), (0.0, 0.0, 0.0, 1.0))
//                 .build()
//             {
//                 state.notify_text = "*** Black button was clicked";
//             }

//             ui.text("This button is red:");
//             if ui
//                 .color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 1.0))
//                 .build()
//             {
//                 state.notify_text = "*** Red button was clicked";
//             }

//             ui.text("This button is BIG because it has a custom size:");
//             if ui
//                 .color_button(im_str!("Green color"), (0.0, 1.0, 0.0, 1.0))
//                 .size((100.0, 50.0))
//                 .build()
//             {
//                 state.notify_text = "*** BIG button was clicked";
//             }

//             ui.text("This button doesn't use the tooltip at all:");
//             if ui
//                 .color_button(im_str!("No tooltip"), (0.0, 0.0, 1.0, 1.0))
//                 .tooltip(false)
//                 .build()
//             {
//                 state.notify_text = "*** No tooltip button was clicked";
//             }
//         });
// }

// fn example_2(ui: &Ui) {
//     ui.window(im_str!("Example 2: Alpha component"))
//         .size((700.0, 320.0), ImGuiCond::Appearing)
//         .position((20.0, 140.0), ImGuiCond::Appearing)
//         .build(|| {
//             ui.text_wrapped(im_str!(
//                 "The displayed color is passed to the button as four float values between \
//                  0.0 - 1.0 (RGBA). If you don't care about the alpha component, it can be \
//                  disabled and it won't show up in the tooltip"
//             ));

//             ui.text("This button ignores the alpha component:");
//             ui.color_button(im_str!("Red color"), (1.0, 0.0, 0.0, 0.5))
//                 .alpha(false)
//                 .build();

//             ui.spacing();
//             ui.spacing();
//             ui.spacing();
//             ui.text_wrapped(im_str!(
//                 "If you *do* care about the alpha component, you can choose how it's \
//                  displayed in the button and the tooltip"
//             ));

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::Opaque (default) doesn't show the alpha component at all"
//             ));
//             ui.color_button(im_str!("Red + ColorPreview::Opaque"), (1.0, 0.0, 0.0, 0.5))
//                 .preview(ColorPreview::Opaque)
//                 .build();

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::HalfAlpha divides the color area into two halves and uses a \
//                  checkerboard pattern in one half to illustrate the alpha component"
//             ));
//             ui.color_button(
//                 im_str!("Red + ColorPreview::HalfAlpha"),
//                 (1.0, 0.0, 0.0, 0.5),
//             ).preview(ColorPreview::HalfAlpha)
//             .build();

//             ui.separator();
//             ui.text_wrapped(im_str!(
//                 "ColorPreview::Alpha uses a checkerboard pattern in the entire color area to \
//                  illustrate the alpha component"
//             ));
//             ui.color_button(im_str!("Red + ColorPreview::Alpha"), (1.0, 0.0, 0.0, 0.5))
//                 .preview(ColorPreview::Alpha)
//                 .build();
//         });
// }