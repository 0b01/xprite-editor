use crate::prelude::*;
use xprite::rendering::Renderer;
use std::borrow::Cow;

const COLORS_PER_ROW: usize = 10;
const MARGIN: f32 = 10.;
const PALETTE_W: f32 = LEFT_SIDE_WIDTH - 2. * MARGIN;
const PALETTE_H: f32 = 400.;
const BLOCK_SZ: f32 = PALETTE_W / COLORS_PER_ROW as f32;
const PALETTE_BEGIN_Y: f32 = 500.;

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
    let pal_name = "pico8";
    ui.set_cursor_screen_pos((MARGIN, PALETTE_BEGIN_Y - 30.));
    ui.text(
        im_str!("Palette: {}", pal_name)
    );
    ui.text(
        im_str!("Color: {}", state.palette_color_name.as_ref().unwrap_or(&Cow::Borrowed("None")))
    );

    let draw_list = ui.get_window_draw_list();
    draw_list.add_rect(
        (MARGIN, PALETTE_BEGIN_Y),
        (RIGHT_SIDE_WIDTH - MARGIN, PALETTE_BEGIN_Y + PALETTE_H),
        LIGHT_GREY
    ).filled(false).build();
    for (i, (col_name, col)) in state.xpr.palette_man.get(pal_name).unwrap().iter().enumerate() {
        let col: [f32; 4] = (*col).into();
        let x = MARGIN + BLOCK_SZ * ((i % COLORS_PER_ROW) as f32);
        let y = PALETTE_BEGIN_Y + BLOCK_SZ * ((i / COLORS_PER_ROW) as f32);
        draw_list.add_rect(
            (x, y),
            (x + BLOCK_SZ, y + BLOCK_SZ ),
            col
        ).filled(true).build();

        ui.set_cursor_screen_pos((x, y));
        if ui.invisible_button(im_str!("colorcell{}{}", col_name, i), (BLOCK_SZ, BLOCK_SZ)) {
            state.xpr.selected_color = col.into();
        }
        if ui.is_item_hovered() {
            state.palette_color_name = Some(Cow::Owned(String::from(*col_name)));
        }
    }

    if ui.small_button(im_str!("test")) {
        info!("CLICKED");
    }
}

fn draw_color_picker(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let mut sel: [f32; 4] = state.xpr.selected_color.into();
    if ui.color_picker(im_str!("color"), &mut sel).build() {
        // println!("{:?}", sel);
        let ret = sel.into();
        // println!("{:?}", ret);
        state.xpr.selected_color = ret;
        // println!("-----------------");
    }
}
