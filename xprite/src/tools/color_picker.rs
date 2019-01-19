use crate::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct ColorPicker {
    cursor: Option<Pixels>,
    col: Option<Color>,
}

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker {
            cursor: None,
            col: None,
        }
    }
}

impl Tool for ColorPicker {
    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel { point, color }));
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &Xprite, _p: Vec2f) -> Result<(), String> {
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, _button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let colors: Vec<_> = xpr
            .history
            .top()
            .groups
            .iter()
            .map(|group| group.1.iter().map(|layer| layer.get_color(point)))
            .flatten()
            .collect();
        let picked = colors.iter().find(|i| i.is_some());
        match picked {
            Some(Some(col)) => {
                self.col = Some(*col);
            }
            Some(None) => panic!("impossible"),
            None => self.col = Some(Color::transparent()),
        }
        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(col) = self.col {
            xpr.set_color(&col);
        }
        self.col = None;
        Ok(false)
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        let _ = self.set_cursor(xpr);
        Ok(false)
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, _value: &str) -> Result<(), String> {
        match option {
            _ => (), // noop
        }
        Ok(())
    }
}
