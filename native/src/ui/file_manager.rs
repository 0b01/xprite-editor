use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_file_manager(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let window_title = im_str!("{}", if state.show_file_is_save {
        "Save file"
    } else {
        "Open file"
    });

    if state.show_file_popup {
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
                state.show_file_popup = false;
            };

            let open = get_callback();

            ui.with_item_width(400., || {
                if ui
                    .input_text(im_str!(""), &mut state.open_file_name)
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
        let fname: &str = state.open_file_name.as_ref();
        info!("opening: {}", fname);
        if fname.ends_with(".ase") || fname.ends_with(".aseprite") {
            state.load_ase(&fname.to_owned());
        } else if fname.ends_with(".png") {
            state.load_png(&fname.to_owned());
        }
    }
}