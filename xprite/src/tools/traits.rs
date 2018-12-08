use crate::prelude::*;

pub trait Tool {
    fn tool_type(&self) -> ToolType;
    fn mouse_move(&mut self, xpr:&mut Xprite, p: Vec2D) -> Option<()>;
    fn mouse_up(&mut self, xpr:&mut Xprite, p: Vec2D) -> Option<()>;
    fn mouse_down(&mut self, xpr:&mut Xprite, p: Vec2D, button: InputItem) -> Option<()>;
    fn draw(&mut self, xpr:&mut Xprite) -> Option<()>;
    fn set(&mut self, xpr:&mut Xprite, option: &str, value: &str) -> Option<()>;
}