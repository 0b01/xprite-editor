use crate::algorithms::rect::*;
use crate::core::outline::outline_rect;
use crate::tools::*;
use std::num::NonZeroU32;
use wfc_image::*;

#[derive(Clone)]
pub struct Texture {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Vec2f>,
    start_pos: Option<Vec2f>,

    // params:
    pub pattern_size: u32,
    pub orientation_reflection: bool,
    pub orientation_rotation: bool,
    pub wrap_x: bool,
    pub wrap_y: bool,
    pub tex_w: i32,
    pub tex_h: i32,

    // texture image
    pub tex: Option<(usize, img::DynamicImage)>,
}

impl Default for Texture {
    fn default() -> Self {
        Self::new()
    }
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            is_mouse_down: None,
            start_pos: None,
            orientation_reflection: true,
            orientation_rotation: true,
            wrap_x: false,
            wrap_y: false,
            cursor_pos: None,
            pattern_size: 3,
            tex_w: 100,
            tex_h: 100,
            tex: None,
        }
    }

    pub fn finalize(&mut self, xpr: &mut Xprite) -> Result<img::DynamicImage, String> {
        self.quilt_img(xpr)
    }

    pub fn get_bb(&self) -> Option<Rect> {
        let (start, stop) = (self.start_pos?, self.cursor_pos?);
        let x1_ = start.x as i32;
        let y1_ = start.y as i32;
        let x2_ = stop.x as i32;
        let y2_ = stop.y as i32;
        let x1 = i32::min(x1_, x2_);
        let x2 = i32::max(x1_, x2_);
        let y1 = i32::min(y1_, y2_);
        let y2 = i32::max(y1_, y2_);
        let bb = Rect(vec2f!(y1, x1), vec2f!(y2 - 1, x2 - 1));
        Some(bb)
    }

    fn quilt_img(&mut self, xpr: &mut Xprite) -> Result<img::DynamicImage, String> {
        let bb = self.get_bb().ok_or("no cursor".to_owned())?;
        let mut content = xpr.current_layer().unwrap().content.clone();
        content.retain_in_rect_mut(bb);
        let img = content.as_image(bb);

        let orientation = {
            let mut ret = vec![Orientation::Original];
            if self.orientation_reflection {
                ret.push(Orientation::DiagonallyFlipped);
                ret.push(Orientation::DiagonallyFlippedClockwise180);
                ret.push(Orientation::DiagonallyFlippedClockwise270);
            }
            if self.orientation_rotation {
                ret.push(Orientation::Clockwise90);
                ret.push(Orientation::Clockwise180);
                ret.push(Orientation::Clockwise270);
            }
            ret
        };

        let pattern_size = NonZeroU32::new(self.pattern_size).expect("pattern size may not be zero");
        let output_size = Size::new(self.tex_w as u32, self.tex_h as u32);
        macro_rules! gen {
            ($e:expr) => {
                generate_image(
                    &img,
                    pattern_size,
                    output_size,
                    &orientation,
                    $e,
                    retry::NumTimes(10),
                )
                .map_err(|_| "Too many contradictions".to_owned())
            }
        };

        let res = match (self.wrap_x, self.wrap_y) {
            (true, true) => gen!(wrap::WrapXY),
            (true, false) => gen!(wrap::WrapX),
            (false, true) => gen!(wrap::WrapY),
            (false, false) => gen!(wrap::WrapNone),
        }?;
        Ok(res)
    }
}

impl Tool for Texture {
    fn cursor(&self) -> Option<Pixels> {
        let p = self.cursor_pos?;
        None
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        if self.is_mouse_down.is_some() {
            self.cursor_pos = Some(point);
        }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.cursor_pos = Some(point);
        // self.quilt_img(xpr)?;

        self.is_mouse_down = None;
        // self.start_pos = None;

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        if InputItem::Left != button {
            return Ok(());
        }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.start_pos = Some(point);
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = self.cursor() {
            xpr.set_cursor(&cursor);
        }
        if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
            xpr.add_marquee(&marq);
            return Ok(true);
        }
        Ok(false)
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "ctrl" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "shift" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => (),
        }
        Ok(())
    }
}
