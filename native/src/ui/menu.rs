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
                state.xpr_mut().undo();
            }
            if ui
                .menu_item(im_str!("Redo"))
                .shortcut(im_str!("Ctrl+y"))
                .build()
            {
                state.xpr_mut().redo();
            }
        });

        ui.menu(im_str!("Window")).build(|| {
            if ui.menu_item(im_str!("Symmetry")).build() {
                state.toggle_symmetry();
            }
            if ui.menu_item(im_str!("Console")).build() {
                state.toggle_console();
            }
            if ui.menu_item(im_str!("Brush")).build() {
                state.toggle_brush();
            }
        });

        ui.menu(im_str!("Documents")).build(|| {
            for (i, x) in state.xprs.iter().enumerate() {
                let mut is_sel = i == state.xpr_idx;
                if ui.menu_item(im_str!("{}", x.name))
                    .selected(&mut is_sel)
                    .build()
                {
                    state.xpr_idx = i;
                }
            }
        });

        if cfg!(debug_assertions) {
            ui.text(im_str!("FPS: {:.2}", ui.framerate()));
        }
    })
}
