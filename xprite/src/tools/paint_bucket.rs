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

    pub const VARIANTS: [PaintBucketMode; 2] = [
        PaintBucketMode::Fill,
        PaintBucketMode::Outline,
    ];
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
    buffer: Option<Pixels>,
    pub mode: PaintBucketMode,
}

impl PaintBucket {
    pub fn new() -> Self {
        PaintBucket {
            cursor: None,
            is_mouse_down: false,
            buffer: None,
            mode: PaintBucketMode::Fill,
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
        let buffer = algorithms::floodfill::floodfill(w, h, pixs, p, bg_color, color);
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
        let bg_color = xpr.current_layer().unwrap().get_color(point);
        match self.mode {
            PaintBucketMode::Fill => {
                self.buffer = Some(self.floodfill(xpr, point, bg_color)?);
            }
            PaintBucketMode::Outline => {
                let buffer = self.floodfill(xpr, point, bg_color)?;
                let perim = {
                    let w = xpr.canvas.art_w;
                    let h = xpr.canvas.art_h;
                    algorithms::perimeter::find_perimeter(w as usize, h as usize, &buffer)
                };
                self.buffer = Some(perim);
            }
        }

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, _button: InputItem) -> Result<(), String> {
        self.is_mouse_down = true;
        let point = xpr.canvas.shrink_size(p);
        let bg_color = xpr.current_layer().unwrap().get_color(point);
        let buffer = self.floodfill(xpr, point, bg_color)?;
        let mut perim = {
            let w = xpr.canvas.art_w;
            let h = xpr.canvas.art_h;
            algorithms::perimeter::find_perimeter(w as usize, h as usize, &buffer)
        };
        perim.push(Pixel {
            point,
            color: xpr.color(),
        });
        self.cursor = Some(perim);
        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        if let Some(pixs) = &self.buffer {
            xpr.history.enter()?;
            xpr.history
                .top_mut()
                .selected_layer_mut()
                .unwrap()
                .content
                .extend(&pixs);
        }
        self.buffer = None;
        Ok(())
    }
    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);
        Ok(())
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
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
