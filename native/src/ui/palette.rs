use crate::prelude::*;
use xprite::rendering::Renderer;
use std::ops::Index;
use std::borrow::Cow;

const COLORS_PER_ROW: usize = 8;

pub fn draw_palette(rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui
    .window(im_str!("Palette"))
    .position((0.,220.), ImGuiCond::Appearing)
    .size((LEFT_SIDE_WIDTH, 800.), ImGuiCond::Appearing)
    .movable(false)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        draw_color_picker(rdr, state, ui);
        draw_cells(rdr, state, ui);
    });
}

fn draw_cells(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let items: Vec<_> = state.xpr.palette_man.palettes.keys()
        .cloned() .map(ImString::new) .collect();
    let refs: Vec<_> = items.iter().map(|s| s.as_ref()).collect();
    ui.combo(im_str!("Palette"), &mut state.palette_idx, &refs[..], -1);
    ui.text(
        im_str!("Color: {}", state.palette_color_name.as_ref().unwrap_or(&Cow::Borrowed("None")))
    );

    let (mut MARGIN, mut PALETTE_BEGIN_Y) = ui.get_cursor_screen_pos();
    MARGIN += 4.5;
    PALETTE_BEGIN_Y += 4.5;
    let PALETTE_W = LEFT_SIDE_WIDTH - 2. * MARGIN;
    let PALETTE_H = 400.;
    let BLOCK_SZ = PALETTE_W / COLORS_PER_ROW as f32;

    let draw_list = ui.get_window_draw_list();
    draw_list.add_rect(
        (MARGIN - 5., PALETTE_BEGIN_Y - 5.),
        (RIGHT_SIDE_WIDTH - MARGIN + 5., PALETTE_BEGIN_Y + PALETTE_H + 5.),
        LIGHT_GREY
    ).filled(false).build();
    for (i, (col_name, col)) in state.xpr.palette_man.palettes.get_index_mut(state.palette_idx as usize).unwrap().1.iter_mut().enumerate() {
        let x = MARGIN + BLOCK_SZ * ((i % COLORS_PER_ROW) as f32);
        let y = PALETTE_BEGIN_Y + BLOCK_SZ * ((i / COLORS_PER_ROW) as f32);
        ui.set_cursor_screen_pos((x, y));
        if ui.invisible_button(im_str!("colorcell##{}", i), (BLOCK_SZ, BLOCK_SZ)) {
            state.xpr.selected_color = *col;
        }
        ui.set_cursor_screen_pos((x, y));
        let misc_flags = {
            let mut f = ImGuiColorEditFlags::empty();
            f.set(ImGuiColorEditFlags::HDR, true);
            f.set(ImGuiColorEditFlags::AlphaPreview, true);
            f.set(ImGuiColorEditFlags::NoOptions, false);
            f.set(ImGuiColorEditFlags::NoInputs, true);
            f.set(ImGuiColorEditFlags::NoLabel, true);
            f.set(ImGuiColorEditFlags::NoPicker, true);
            f
        };

        let mut sel: [f32; 4] = (*col).into();
        let id = im_str!("MyColor##{}", i);
        let b = ui.color_edit(id, &mut sel)
            .flags(misc_flags)
            .alpha(false);

        if b.build() {
            *col = sel.into();
        }
        if ui.is_item_hovered() {
            state.palette_color_name = Some(Cow::Owned(col_name.to_owned()));
            ui.tooltip(|| {
                ui.text(col_name.to_owned());
            });
        }
    }

    if ui.small_button(im_str!("end")) {
        info!("end pressed");
    }
}

fn draw_color_picker(_rdr: &Renderer, state: &mut State, ui: &Ui) {

    let misc_flags = {
        let mut f = ImGuiColorEditFlags::empty();
        f.set(ImGuiColorEditFlags::AlphaBar, true);
        f.set(ImGuiColorEditFlags::AlphaPreview, true);
        f.set(ImGuiColorEditFlags::AlphaPreviewHalf, true);
        f.set(ImGuiColorEditFlags::NoLabel, true);
        f.set(ImGuiColorEditFlags::HEX, true);

        f.set(ImGuiColorEditFlags::PickerHueBar, true);
        // f.set(ImGuiColorEditFlags::PickerHueWheel, true);
        f
    };

    let mut sel: [f32; 4] = state.xpr.selected_color.into();
    let mut b = ui
        .color_picker(im_str!("MyColor##4"), &mut sel)
        .flags(misc_flags)
        .alpha(true)
        .alpha_bar(true)
        .side_preview(true)
        .rgb(true);

    if b.build() {
        let ret = sel.into();
        state.xpr.selected_color = ret;
    };
}
