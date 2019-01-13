use crate::prelude::*;

pub trait Tool {
    fn tool_type(&self) -> ToolType;
    fn mouse_move(&mut self, xpr:&Xprite, p: Vec2D) -> Result<(), String>;
    fn mouse_up(&mut self, xpr:&Xprite, p: Vec2D) -> Result<(), String>;
    fn mouse_down(&mut self, xpr:&Xprite, p: Vec2D, button: InputItem) -> Result<(), String>;
    fn draw(&mut self, xpr:&mut Xprite) -> Result<(), String>;
    fn update(&mut self, xpr:&mut Xprite) -> Result<(), String> { Ok(()) }
    fn set(&mut self, xpr:&Xprite, option: &str, value: &str) -> Result<(), String>;
    fn cursor(&self) -> Option<Pixels>;
    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        let cursor = self.cursor()?;
        xpr.set_cursor(&cursor);
        Some(())
    }
}