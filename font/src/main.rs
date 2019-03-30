extern crate xprite;
extern crate bdf;
use xprite::prelude::*;
use bdf::{Font, save, BoundingBox, Size,  Glyph, Bitmap};
use std::collections::HashMap;

fn main() {
    // load layers
    let xpr = Xprite::load_ase("font.aseprite");
    let layers = &xpr.history.top().groups.get(0).unwrap().1;

    let mut map: HashMap<char, Pixels> = HashMap::new();

    for layer in layers {
        let layer_name = &layer.name;
        let name = layer_name.chars().next().expect("no char");
        let content = &layer.content;
        if layer_name.len() == 1
        && (
               (name >= 'a' && name <= 'z')
            || (name >= 'A' && name <= 'Z')
        ) {
            map.insert(name, content.clone());
        }
    }

    dbg!(&map);

    let mut font = Font::new("font", None);
    font.set_bounds(BoundingBox{
        width: 10,
        height: 10,
        x: 0,
        y: 0,
    });
    font.set_format("2.1");
    font.set_size(Size{
        pt: 30,
        x: 75,
        y: 75,
    });

    let glyphs = map.into_iter().map(|(k,v)| {
        let mut glyph = Glyph::new(format!("{}",k), k);
        glyph.set_map(to_bitmap(v));
        glyph.set_bounds(BoundingBox{
            width: 10,
            height: 10,
            x: 0,
            y: 0,
        });
        (k, glyph)
    }).collect();

    *font.glyphs_mut() = glyphs;

    assert!(font.validate());
    println!("test");
    dbg!(&font);
    save("test.bdf", &font).unwrap();
}

fn to_bitmap(pixs: Pixels) -> Bitmap {
    let bb  = pixs.bounding_rect();
    let mut bmp = Bitmap::new(bb.w() as u32, bb.h() as u32);
    for (i, row) in pixs.as_mat_bb(bb).iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            bmp.set(j as u32, i as u32, col.is_some());
        }
    }
    bmp
}