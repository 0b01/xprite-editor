use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_file_manager(_rdr: &dyn Renderer, state: &mut State, ui: &Ui) {
    let window_title = im_str!("{}", if state.file_popup.show_file_is_save { "Save file" } else { "Open file" });

    if state.file_popup.show_file_popup {
        ui.open_popup(&window_title);
    }

    ui.popup_modal(&window_title)
        .inputs(true)
        .collapsible(true)
        .resizable(false)
        .movable(true)
        .build(|| {
            let result = if state.file_popup.show_file_is_save {
                nfd::open_save_dialog(None, None).unwrap_or_else(|e| {
                    panic!(e);
                })
            } else {
                nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                    panic!(e);
                })
            };
            match result {
                nfd::Response::Okay(fname) => {
                    info!("File path = {:?}", fname);
                    let save = state.file_popup.show_file_is_save;
                    if fname.ends_with(".ase") || fname.ends_with(".aseprite") {
                        if save {
                            state.xpr().save_ase(&fname);
                        } else {
                            state.push_xpr(Xprite::load_ase(&fname.to_owned()));
                        }
                    } else if fname.ends_with(".png") || fname.ends_with(".jpg") || fname.ends_with(".jpeg") {
                        if save {
                            state.xpr_mut().save_img(&fname.to_owned(), 1);
                        } else {
                            state.push_xpr(Xprite::load_img(&fname.to_owned()));
                        }
                    } else {
                        info!("unimplemented file format {}", &fname);
                    }
                }
                nfd::Response::OkayMultiple(files) => println!("Files {:?}", files),
                nfd::Response::Cancel => println!("User canceled"),
            }
            ui.close_current_popup();
            state.file_popup.show_file_popup = false;

            // let open_file = |state: &mut State| {
            //     let fname = state.file_popup.open_file_name.to_str().to_owned();
            //     info!("opening: {:?}", fname);
            // };

            // let close_window = |state: &mut State| {
            //     state.toggle_hotkeys();
            //     ui.close_current_popup();
            //     state.file_popup.show_file_popup = false;
            // };

            // let _ = ui.push_item_width(400.);
            // if ui
            //     .input_text(&im_str!(""), &mut state.file_popup.open_file_name)
            //     .auto_select_all(true)
            //     .enter_returns_true(true)
            //     .build(&ui)
            // {
            //     open_file(state);
            //     close_window(state);
            // }

            // if ui.button(&im_str!("Cancel"), [60., 20.]) {
            //     close_window(state);
            // }

            // ui.same_line(100.);

            // if ui.button(&window_title, [60., 20.]) {
            //     open_file(state);
            //     close_window(state);
            // }
        });
}
