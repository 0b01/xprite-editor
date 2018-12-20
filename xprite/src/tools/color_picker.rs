use crate::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct ColorPicker { }

impl ColorPicker {
    pub fn new() -> Self {
        ColorPicker { }
    }
}

impl Tool for ColorPicker {

    fn tool_type(&self) -> ToolType {
        ToolType::ColorPicker
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        xpr.set_cursor(&(Pixel {point, color}).into());
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &mut Xprite, _p: Vec2D) -> Result<(), String> {
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, _button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let colors : Vec<_> = xpr.history.top_mut().layers.iter().map(|layer| layer.get_color(point)).collect();
        let picked = colors.iter().find(|i| i.is_some());
        match picked {
            Some(Some(col)) => { xpr.set_color(col); }
            Some(None) => panic!("impossible"),
            None => (),
        }
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
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
