use crate::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct ColorPicker {
    cursor: Option<Pixels>,
}

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker {
            cursor: None,
        }
    }
}

impl Tool for ColorPicker {

    fn tool_type(&self) -> ToolType {
        ToolType::ColorPicker
    }

    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel{point, color}));
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &mut Xprite, _p: Vec2D) -> Result<(), String> {
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, _button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let colors : Vec<_> =
            xpr.history.top_mut()
                .groups.iter()
                .map(|group|
                    group.1.iter().map(|layer| layer.get_color(point))
                ).flatten().collect();
        let picked = colors.iter().find(|i| i.is_some());
        match picked {
            Some(Some(col)) => { xpr.set_color(col); }
            Some(None) => panic!("impossible"),
            None => {xpr.set_color( &Color::transparent() )},
        }
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr).unwrap();
        // noop
        Ok(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, _value: &str) -> Result<(), String> {
        match option {
            _ => (), // noop
        }
        Ok(())
    }
}
