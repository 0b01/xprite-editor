use crate::prelude::*;

#[derive(Clone)]
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

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        xpr.set_cursor(&(Pixel {point, color}).into());
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        Some(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, _button: InputItem) -> Option<()> {
        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        // noop
        Some(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, _value: &str) -> Option<()> {
        match option {
            _ => (), // noop
        }
        Some(())
    }
}
