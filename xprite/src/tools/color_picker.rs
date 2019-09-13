use crate::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct ColorPicker {
    cursor: Option<Pixels>,
    hovered_color: Option<Color>,
    clicked_color: Option<Color>,
}

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker {
            cursor: None,
            hovered_color: None,
            clicked_color: None,
        }
    }

    fn find_color(&self, xpr: &Xprite, point: Vec2f) -> Option<Color> {
        let colors: Vec<_> = xpr
            .frame()
            .groups
            .iter()
            .map(|group| group.1.iter().filter_map(|layer| if layer.borrow().visible { Some(layer.borrow().get_color(point)) } else { None }))
            .flatten()
            .collect();
        *colors.iter().find(|i| i.is_some())?
    }
}

impl Tool for ColorPicker {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel { point, color }));
        self.hovered_color = if let Some(col) = self.find_color(xpr, point) {
            Some(col)
        } else {
            Some(Color::transparent())
        };
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &mut Xprite, _p: Vec2f) -> Result<(), String> {
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, _button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.clicked_color = if let Some(col) = self.find_color(xpr, point) {
            Some(col)
        } else {
            Some(Color::transparent())
        };
        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(hovered_color) = self.hovered_color {
            // order is important
            xpr.color_picker_color = Some(hovered_color);
        }
        if let Some(col) = self.clicked_color {
            xpr.palette.set_color(col);
            xpr.color_picker_color = None;
        }
        self.clicked_color = None;
        self.hovered_color = None;
        Ok(false)
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = &self.cursor {
            xpr.set_cursor(cursor);
        }
        Ok(false)
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, _value: &str) -> Result<(), String> {
        match option {
            _ => (), // noop
        }
        Ok(())
    }
}
