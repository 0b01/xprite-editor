use crate::algorithms;
use crate::prelude::*;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PaintBucketMode {
    Fill,
    Outline,
}

impl PaintBucketMode {
    pub fn as_str(&self) -> &str {
        match self {
            PaintBucketMode::Fill => "Fill",
            PaintBucketMode::Outline => "Outline",
        }
    }

    pub const VARIANTS: [PaintBucketMode; 2] =
        [PaintBucketMode::Fill, PaintBucketMode::Outline];
}

impl FromStr for PaintBucketMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Fill" => Ok(PaintBucketMode::Fill),
            "Outline" => Ok(PaintBucketMode::Outline),
            _ => Err(()),
        }
    }
}

impl Default for PaintBucketMode {
    fn default() -> Self {
        PaintBucketMode::Fill
    }
}

#[derive(Clone, Debug, Default)]
pub struct PaintBucket {
    cursor: Option<Pixels>,
    is_mouse_down: bool,
    update_buffer: Option<Pixels>,
    draw_buffer: Option<Pixels>,
    pub mode: PaintBucketMode,
}

impl PaintBucket {
    pub fn new() -> Self {
        PaintBucket {
            cursor: None,
            is_mouse_down: false,
            update_buffer: None,
            mode: PaintBucketMode::Fill,
            draw_buffer: None,
        }
    }

    pub fn floodfill(
        &self,
        xpr: &Xprite,
        p: Vec2f,
        bg_color: Option<Color>,
    ) -> Result<Pixels, String> {
        let color = xpr.color();
        let w = xpr.canvas.art_w;
        let h = xpr.canvas.art_h;
        let current_layer = xpr.current_layer().unwrap();
        let pixs = &current_layer.content;
        let buffer =
            algorithms::floodfill::floodfill(w, h, pixs, p, bg_color, color);
        // info!{"{:#?}", buffer};
        Ok(buffer)
    }
}

impl Tool for PaintBucket {
    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down {
            return self.mouse_down(xpr, p, InputItem::Left);
        }
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel { point, color }));
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        self.is_mouse_down = false;

        // reset cursor
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel { point, color }));

        let (w, h) = (xpr.canvas.art_w, xpr.canvas.art_h);
        if oob(point.x, point.y, w, h) {
            return Ok(());
        }
        std::mem::swap(&mut self.update_buffer, &mut self.draw_buffer);

        Ok(())
    }

    fn mouse_down(
        &mut self,
        xpr: &Xprite,
        p: Vec2f,
        _button: InputItem,
    ) -> Result<(), String> {
        self.is_mouse_down = true;
        let point = xpr.canvas.shrink_size(p);
        let bg_color = xpr.current_layer().unwrap().get_color(point);

        let ff = self.floodfill(xpr, point, bg_color)?;
        self.draw_buffer = match self.mode {
            PaintBucketMode::Fill => Some(ff),
            PaintBucketMode::Outline => {
                let perim = {
                    let w = xpr.canvas.art_w;
                    let h = xpr.canvas.art_h;
                    algorithms::perimeter::find_perimeter(
                        w as usize, h as usize, &ff,
                    )
                };
                Some(perim)
            }
        };

        self.cursor = Some(pixels!(Pixel {
            point,
            color: xpr.color()
        }));
        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.history.enter()?;
            xpr.history
                .top_mut()
                .selected_layer_mut()
                .unwrap()
                .content
                .extend(&pixs);
            self.update_buffer = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = self.cursor() {
            xpr.set_cursor(&cursor);
        }
        if let Some(pixs) = &self.draw_buffer {
            // pixs.set_color(xpr.color());
            xpr.add_pixels(&pixs);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(
        &mut self,
        _xpr: &Xprite,
        option: &str,
        value: &str,
    ) -> Result<(), String> {
        match option {
            "mode" => {
                use self::PaintBucketMode::*;
                match PaintBucketMode::from_str(value) {
                    Ok(Fill) => self.mode = Fill,
                    Ok(Outline) => self.mode = Outline,
                    _ => (),
                };
            }
            _ => (),
        };
        Ok(())
    }
}
