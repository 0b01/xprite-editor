use crate::algorithms::symmetry::SymmetryMode;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct Symmetry {
    is_mouse_down: Option<InputItem>,
    /// (enabled, symm)
    pub symms: Vec<(bool, SymmetryMode)>,
    pub dirty: bool,
}

impl Symmetry {
    pub fn new() -> Self {
        Symmetry {
            is_mouse_down: None,
            symms: vec![],
            dirty: false,
        }
    }

    pub fn add_symmetry(&mut self, symm: SymmetryMode) {
        self.dirty = true;
        self.symms.push((true, symm));
    }

    pub fn remove_symmetry(&mut self, i: usize) {
        self.dirty = true;
        self.symms.remove(i);
    }

    /// returns reflected stroke
    pub fn process(&self, pixs: &Pixels) -> Pixels {
        if self.symms.is_empty() {
            return Pixels::new();
        }
        let mut ret = Pixels::new();
        if self.symms[0].0 {
            self.symms[0].1.process(pixs, &mut ret);
        }
        for (to_process, symm) in &self.symms[1..] {
            if !to_process {
                continue;
            }
            symm.process(&ret.clone(), &mut ret);
            symm.process(pixs, &mut ret);
        }
        ret
    }
}

impl Tool for Symmetry {
    fn mouse_move(&mut self, _xpr: &Xprite, _p: Vec2f) -> Result<(), String> {
        // // set current cursor_pos
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // if self.is_mouse_down.is_some() {
        //     self.cursor_pos = Some(Pixel { point, color });
        // }
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &mut Xprite, _p: Vec2f) -> Result<(), String> {
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // self.cursor_pos = Some(Pixel { point, color });

        // self.is_mouse_down = None;
        // // self.start_pos = None;
        Ok(())
    }

    fn mouse_down(&mut self, _xpr: &Xprite, _p: Vec2f, _button: InputItem) -> Result<(), String> {
        // if InputItem::Left != button {
        //     return Ok(());
        // }
        // self.is_mouse_down = Some(button);
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // self.start_pos = Some(Pixel { point, color });
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        // if let Some(cursor) = None {
        //     xpr.set_cursor(&cursor);
        // }
        // if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
        //     xpr.add_marquee(&marq);
        // }
        Ok(false)
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if self.dirty {
            let (w, h) = xpr.canvas.get_art_dimension();
            let lines = self
                .symms
                .iter()
                .filter_map(|(show, symmetry)| if *show { Some(symmetry.auxiliary_line(w, h)) } else { None })
                .flatten()
                .collect();
            xpr.update_lines(lines);
            self.dirty = false;
            Ok(true)
        } else {
            Ok(false)
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_multistep_symmetry() {
        // this test is functionally equivalent to the prevoius test
        use super::*;
        let pixs = pixels!(pixel!(0, 0, Color::red()), pixel!(1, 0, Color::red()));
        let mut symm = Symmetry::new();
        symm.add_symmetry(SymmetryMode::Horizontal(2.));
        symm.add_symmetry(SymmetryMode::Vertical(1.));
        let ret = symm.process(&pixs);
        assert_eq!(
            ret,
            pixels!(
                pixel!(0, 1, Color::red()),
                pixel!(1, 1, Color::red()),
                pixel!(2, 0, Color::red()),
                pixel!(3, 0, Color::red()),
                pixel!(2, 1, Color::red()),
                pixel!(3, 1, Color::red())
            )
        );
    }
}
