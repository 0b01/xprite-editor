use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_menu(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            if ui
                .menu_item(im_str!("Load"))
                .shortcut(im_str!("Ctrl+O"))
                .build()
            {
                state.execute(Bind::Load).unwrap();
            }
            if ui
                .menu_item(im_str!("Save"))
                .shortcut(im_str!("Ctrl+S"))
                .build()
            {
                state.execute(Bind::Save).unwrap();
            }
            if ui.menu_item(im_str!("Settings")).build() {
                state.execute(Bind::PushTool(ToolType::Settings)).unwrap();
            }

            if ui.menu_item(im_str!("Exporter")).build() {
                state.toggle_exporter();
            }

        });

        ui.menu(im_str!("Edit")).build(|| {
            if ui
                .menu_item(im_str!("Undo"))
                .shortcut(im_str!("Ctrl+Z"))
                .build()
            {
                state.xpr.undo();
            }
            if ui
                .menu_item(im_str!("Redo"))
                .shortcut(im_str!("Ctrl+y"))
                .build()
            {
                state.xpr.redo();
            }

            if ui.menu_item(im_str!("Symmetry")).build() {
                state.toggle_symmetry();
            }
        });

        if cfg!(debug_assertions) {
            ui.text(im_str!("FPS: {:.2}", ui.framerate()));
        }
    })
}
