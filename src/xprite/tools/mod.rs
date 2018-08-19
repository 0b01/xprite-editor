use xprite::Xprite;
use stdweb::web::event::MouseButton;

pub mod pencil;

pub trait Tool {
    fn get_name(&self) -> &'static str;
    fn mouse_move(&mut self, &mut Xprite, x: i32, y: i32);
    fn mouse_up(&mut self, &mut Xprite, x: i32, y: i32);
    fn mouse_down(&mut self, &mut Xprite, x: i32, y: i32, button: MouseButton);
    fn draw(&self, &Xprite);
    fn set(&mut self, option: &str, value: &str);
}
