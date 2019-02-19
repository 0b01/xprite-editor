use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_file_manager(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let window_title = im_str!(
        "{}",
        if state.file_popup.show_file_is_save {
            "Save file"
        } else {
            "Open file"
        }
    );

    if state.file_popup.show_file_popup {
        ui.open_popup(window_title);
    }

    ui.popup_modal(window_title)
        .inputs(true)
        .collapsible(true)
        .resizable(false)
        .movable(true)
        .build(|| {
            let close = |state: &mut State| {
                state.toggle_hotkeys();
                ui.close_current_popup();
                state.file_popup.show_file_popup = false;
            };

            let open = get_callback();

            ui.with_item_width(400., || {
                if ui
                    .input_text(
                        im_str!(""),
                        &mut state.file_popup.open_file_name,
                    )
                    .auto_select_all(true)
                    .enter_returns_true(true)
                    .build()
                {
                    open(state);
                    close(state);
                }

                if ui.button(im_str!("Cancel"), (60., 20.)) {
                    close(state);
                }

                ui.same_line(100.);

                if ui.button(im_str!("Open"), (60., 20.)) {
                    open(state);
                    close(state);
                }
            });
        });
}

fn get_callback() -> impl Fn(&mut State) {
    |state: &mut State| {
        let fname = state.file_popup.open_file_name.to_str().to_owned();
        info!("opening: {:?}", fname);
        let save = state.file_popup.show_file_is_save;
        if fname.ends_with(".ase") || fname.ends_with(".aseprite") {
            if save {
                state.xpr().save_ase(&fname);
            } else {
                state.xprs.push(Xprite::load_ase(&fname.to_owned()));
            }
        } else if fname.ends_with(".png")
            || fname.ends_with(".jpg")
            || fname.ends_with(".jpeg")
        {
            if save {
                state.xpr_mut().save_img(&fname.to_owned(), 1);
            } else {
                state.xprs.push(Xprite::load_img(&fname.to_owned()));
            }
        } else {
            info!("unimplemented file format {}", &fname);
        }
    }
}
