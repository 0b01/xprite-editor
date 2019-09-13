use crate::core::outline::outline_rect;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct Marquee {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Vec2f>,
    start_pos: Option<Vec2f>,

    move_orig_pos: Option<Vec2f>,
    move_final_pos: Option<Vec2f>,
}

impl Marquee {
    pub fn new() -> Self {
        Marquee {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
            move_orig_pos: None,
            move_final_pos: None,
        }
    }

    pub fn get_bb(&self) -> Option<Rect> {
        let (start, stop) = (self.start_pos?, self.cursor_pos?);
        let x1_ = start.x as i32;
        let y1_ = start.y as i32;
        let x2_ = stop.x as i32;
        let y2_ = stop.y as i32;
        let x1 = i32::min(x1_, x2_);
        let x2 = i32::max(x1_, x2_);
        let y1 = i32::min(y1_, y2_);
        let y2 = i32::max(y1_, y2_);
        let bb = Rect(vec2f!(y1, x1), vec2f!(y2 - 1, x2 - 1));
        Some(bb)
    }

    fn move_pixs(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        dbg!("move_pixs");
        let start = self.start_pos.ok_or("start".to_owned())?;
        let cursor = self.cursor_pos.ok_or("stop".to_owned())?;
        let move_orig = self.move_orig_pos.ok_or("move_orig".to_owned())?;
        let move_final = self.move_final_pos.ok_or("move_final".to_owned())?;
        let diff = move_final - move_orig;

        xpr.commit();

        let bb = Rect(start, cursor);
        let mut pixs = xpr.cel().unwrap().borrow().content.clone();
        pixs.retain_in_rect_mut(bb);

        let l = xpr.cel().unwrap();
        let content_mut = &mut l.borrow_mut().content;
        content_mut.sub_mut(&pixs);
        content_mut.extend(&pixs.shifted(diff));

        self.move_orig_pos = None;
        self.move_final_pos = None;

        self.start_pos = self.start_pos.map(|i| i + diff);
        self.cursor_pos = self.cursor_pos.map(|i| i + diff);

        Ok(())
    }
}

impl Tool for Marquee {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        if self.move_orig_pos.is_some() {
            return Ok(());
        }
        if self.is_mouse_down.is_some() {
            self.cursor_pos = Some(point);
        }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        if self.move_orig_pos.is_some() {
            self.move_final_pos = Some(point);
        } else {
            self.cursor_pos = Some(point);
        }
        self.is_mouse_down = None;
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        if InputItem::Left != button {
            return Ok(());
        }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);

        if self.start_pos.is_some() && self.cursor_pos.is_some() && {
            let bb = self.get_bb().unwrap();
            !oob(point.x - bb.0.x, point.y - bb.0.y, bb.w(), bb.h())
        } {
            self.move_orig_pos = Some(point);
            return Ok(());
        }

        self.start_pos = Some(point);
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = None {
            xpr.set_cursor(&cursor);
        }
        if self.start_pos.is_some() && self.cursor_pos.is_some() {
            if let Ok(marq) = outline_rect(self.start_pos.unwrap(), self.cursor_pos.unwrap()) {
                xpr.add_marquee(&marq);
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        let ret = self.move_pixs(xpr);
        Ok(ret.is_ok())
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "LControl" | "RControl" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "LShift" | "RShift" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => (),
        }
        Ok(())
    }
}
