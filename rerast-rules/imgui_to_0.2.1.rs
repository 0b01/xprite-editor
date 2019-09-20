use imgui::{self, *};

fn replace_ui_selectable(ui: &Ui, imstr: imgui::ImString, selected: bool, flags: imgui::ImGuiSelectableFlags, size: [f32; 2]) {
    replace!(
            ui.selectable(&imstr, selected, flags, size)
        =>
            Selectable::new(&imstr)
                .selected(selected)
                .flags(flags)
                .size(size)
                .build(&ui)
    )
}


fn replace_ui_selectable2(ui: &Ui, imstr: &imgui::ImStr, selected: bool, flags: imgui::ImGuiSelectableFlags, size: [f32; 2]) {
    replace!(
            ui.selectable(&imstr, selected, flags, size)
        =>
            Selectable::new(&imstr)
                .selected(selected)
                .flags(flags)
                .size(size)
                .build(&ui)
    )
}


fn replace_image(ui: &Ui, ui2: &Ui, tid: TextureId, size: [f32; 2]) {
    replace!(
        ui.image(tid, size).build(&ui2)
        =>
        Image::new(tid, size).build(&ui)
    )
}