use crate::prelude::*;
use crate::algorithms;

#[derive(Clone, Default, Debug)]
pub struct PaintBucket {
    cursor: Option<Pixels>,
    is_mouse_down: bool,
}

impl PaintBucket {
    pub fn new() -> Self {
        PaintBucket {
            cursor: None,
            is_mouse_down: false,
        }
    }

    pub fn floodfill(&self, xpr: &mut Xprite, p: Vec2D, bg_color: Option<Color>) -> Result<Pixels, String> {
        let color = xpr.color();
        let w = xpr.canvas.art_w;
        let h = xpr.canvas.art_h;
        let current_layer = xpr.current_layer_mut().unwrap();
        let pixs = &current_layer.content;
        let buffer = algorithms::floodfill::floodfill(w, h, pixs, p, bg_color, color);
        // info!{"{:#?}", buffer};
        Ok(buffer)
    }

}

impl Tool for PaintBucket {

    fn tool_type(&self) -> ToolType {
        ToolType::PaintBucket
    }

    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        if self.is_mouse_down { return self.mouse_down(xpr, p, InputItem::Left) }
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel {point, color}));
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        self.is_mouse_down = false;

        // reset cursor
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor = Some(pixels!(Pixel {point, color}));

        let (w, h) = (xpr.canvas.art_w, xpr.canvas.art_h);
        if oob(point.x, point.y, w, h) { return Ok(()); }
        let bg_color = xpr.current_layer().unwrap().get_color(point);
        let buffer = self.floodfill(xpr, point, bg_color)?;

        xpr.history.enter()?;
        xpr.history.top_mut()
            .selected_layer_mut().unwrap()
            .content
            .extend(&buffer);
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, _button: InputItem) -> Result<(), String> {
        self.is_mouse_down = true;
        let point = xpr.canvas.shrink_size(p);
        let bg_color = xpr.current_layer().unwrap().get_color(point);
        let buffer = self.floodfill(xpr, point, bg_color)?;
        let mut perim = {
            let w = xpr.canvas.art_w;
            let h = xpr.canvas.art_h;
            algorithms::perimeter::find_perimeter(w as usize, h as usize, &buffer)
        };
        perim.push(Pixel{point, color: xpr.color()});
        self.cursor = Some(perim);
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);
        Ok(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, _value: &str) -> Result<(), String> {
        match option {
            _ => (), // noop
        }
        Ok(())
    }
}
