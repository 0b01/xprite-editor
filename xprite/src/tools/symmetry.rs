use crate::tools::*;
#[derive(Clone, Debug)]
pub enum SymmetryMode {
    /// -
    Horizontal(f64),
    /// |
    Vertical(f64),
    /// / parameterized by the y intercept
    AntiDiagonal(f64),
    /// \
    Diagonal(f64),
    /// (horizontal, vertical)
    Quad(f64, f64),
}

impl SymmetryMode {
    pub fn as_str(&self) -> &str {
        match self {
            SymmetryMode::Horizontal(_) => "Horizontal",
            SymmetryMode::Vertical(_) => "Vertical",
            SymmetryMode::Diagonal(_) => "Diagonal",
            SymmetryMode::AntiDiagonal(_) => "AntiDiagonal",
            SymmetryMode::Quad(_,_) => "Quad",
        }
    }

    pub const VARIANTS: [SymmetryMode; 5] =
        [
            SymmetryMode::Horizontal(0.),
            SymmetryMode::Vertical(0.),
            SymmetryMode::Quad(0., 0.),
            SymmetryMode::Diagonal(0.),
            SymmetryMode::AntiDiagonal(0.),
        ];

    pub fn process(&self, pixs: &Pixels, ret: &mut Pixels) {
        match self {
            SymmetryMode::Horizontal(m) => {
                let adjust = 1.; // ...
                for Pixel{point: Vec2f{x,y}, color} in pixs.iter() {
                    ret.push(pixel_xy!(*x, m - (y - m + adjust), *color));
                }
            }
            SymmetryMode::Vertical(m) => {
                let adjust = 1.; // ...
                for Pixel{point: Vec2f{x,y}, color} in pixs.iter() {
                    ret.push(pixel_xy!(m - (x - m + adjust), *y, *color));
                }
            }
            SymmetryMode::Quad(m1, m2) => {
                SymmetryMode::Horizontal(*m1).process(pixs, ret);
                SymmetryMode::Vertical(*m2).process(&ret.clone(), ret); // ...
                SymmetryMode::Vertical(*m2).process(pixs, ret);
            }
            SymmetryMode::AntiDiagonal(y) => {
                let pivot = vec2f!(0, *y);
                SymmetryMode::Vertical(*y).process(pixs, ret);
                *ret = ret.rotate(pivot, -PI/2.);
            }
            SymmetryMode::Diagonal(y) => {
                let pivot = vec2f!(0, *y);
                SymmetryMode::Horizontal(*y).process(pixs, ret);
                *ret = ret.rotate(pivot, -PI/2.);
            }
        }
    }
}

impl Default for SymmetryMode {
    fn default() -> Self {
        SymmetryMode::Vertical(0.)
    }
}


#[derive(Clone, Default, Debug)]
pub struct Symmetry {
    is_mouse_down: Option<InputItem>,
    pub steps: Vec<SymmetryMode>,
}

impl Symmetry {
    pub fn new() -> Self {
        Symmetry {
            is_mouse_down: None,
            steps: vec![],
        }
    }

    pub fn push(&mut self, symm: SymmetryMode) {
        self.steps.push(symm);
    }

    /// returns reflected stroke
    pub fn process(&self, pixs: &Pixels) -> Pixels {
        if self.steps.is_empty() { return Pixels::new(); }
        let mut ret = Pixels::new();
        self.steps[0].process(pixs, &mut ret);
        for symm in &self.steps[1..] {
            symm.process(&ret.clone(), &mut ret);
            symm.process(pixs, &mut ret);
        };
        ret
    }
}

impl Tool for Symmetry {
    fn cursor(&self) -> Option<Pixels> {
        // let p = self.cursor_pos?;
        // Some(pixels!(p))
        None
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // // set current cursor_pos
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // if self.is_mouse_down.is_some() {
        //     self.cursor_pos = Some(Pixel { point, color });
        // }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // self.cursor_pos = Some(Pixel { point, color });

        // self.is_mouse_down = None;
        // // self.start_pos = None;
        Ok(())
    }

    fn mouse_down(
        &mut self,
        xpr: &Xprite,
        p: Vec2f,
        button: InputItem,
    ) -> Result<(), String> {
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
        // if let Some(cursor) = self.cursor() {
        //     xpr.set_cursor(&cursor);
        // }
        // if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
        //     xpr.add_marquee(&marq);
        // }
        Ok(false)
    }

    fn set(
        &mut self,
        _xpr: &Xprite,
        option: &str,
        value: &str,
    ) -> Result<(), String> {
        match option {
            "ctrl" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "shift" => match value {
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
    fn test_process_symmetry_vertial() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red()),
            pixel!(2,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Vertical(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,1,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_horizontal() {
        use super::*;
        let pixs = pixels!(
            pixel_xy!(0,0,Color::red()),
            pixel_xy!(1,0,Color::red()),
            pixel_xy!(2,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Horizontal(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel_xy!(0,1,Color::red()),
            pixel_xy!(1,1,Color::red()),
            pixel_xy!(2,1,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_quad() {
        use super::*;
        let pixs = pixels!(
            pixel_xy!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Quad(1., 1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel_xy!(0,1,Color::red()),
            pixel_xy!(1,1,Color::red()),
            pixel_xy!(1,0,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_quad2() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Quad(2., 1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,0,Color::red()),
            pixel!(3,0,Color::red()),
            pixel!(2,1,Color::red()),
            pixel!(3,1,Color::red())
        ));
    }


    #[test]
    fn test_multistep_symmetry() {
        // this test is functionally equivalent to the prevoius test
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut symm = Symmetry::new();
        symm.push(SymmetryMode::Horizontal(2.));
        symm.push(SymmetryMode::Vertical(1.));
        let ret = symm.process(&pixs);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,0,Color::red()),
            pixel!(3,0,Color::red()),
            pixel!(2,1,Color::red()),
            pixel!(3,1,Color::red())
        ));
    }

    #[test]
    fn test_antidiagonal_symmetry() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(2.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(1,1,Color::red())
        ));
    }

    #[test]
    fn test_antidiagonal_symmetry2() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(3.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(2,2,Color::red())
        ));
    }

    #[test]
    fn test_antidiagonal_symmetry3() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(3.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(2,1,Color::red()),
            pixel!(2,2,Color::red())
        ));
    }

    #[test]
    fn test_diagonal_symmetry() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,1,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Diagonal(0.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(1,0,Color::red())
        ));
    }

    #[test]
    fn test_diagonal_symmetry1() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,1,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Diagonal(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,-1,Color::red())
        ));
    }

}