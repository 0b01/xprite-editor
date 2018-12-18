use crate::prelude::*;

pub fn rect(x1: u32, y1: u32, x2:u32, y2:u32, col: Color) -> Pixels {
    let mut ret = Pixels::new();

    for i in x1..x2 {
        ret.push(pixel!(i,y1, col));
        ret.push(pixel!(i,y2-1, col));
    }
    for j in y1..y2 {
        ret.push(pixel!(x1, j, col));
        ret.push(pixel!(x2-1, j, col));
    }

    ret
}

pub fn filled_rect(x1: u32, y1: u32, x2: u32, y2: u32, col: Color) -> Pixels {
    let mut ret = Pixels::new();

    for i in x1..x2 {
        for j in y1..y2 {
            ret.push(pixel!(i,j, col));
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filled() {
        use super::*;
        let rect = filled_rect(0, 0, 3, 3, Color::red());
        let mut exp = Pixels::new();
        exp.push(pixel!(0,0, Color::red()));
        exp.push(pixel!(0,1, Color::red()));
        exp.push(pixel!(0,2, Color::red()));
        exp.push(pixel!(1,0, Color::red()));
        exp.push(pixel!(1,1, Color::red()));
        exp.push(pixel!(1,2, Color::red()));
        exp.push(pixel!(2,0, Color::red()));
        exp.push(pixel!(2,1, Color::red()));
        exp.push(pixel!(2,2, Color::red()));
        assert_eq!(exp, rect);
    }

    #[test]
    fn test_unfilled() {
        use super::*;
        let rect = rect(0, 0, 3, 3, Color::red());
        let mut exp = Pixels::new();
        exp.push(pixel!(0,0, Color::red()));
        exp.push(pixel!(0,2, Color::red()));
        exp.push(pixel!(1,0, Color::red()));
        exp.push(pixel!(1,2, Color::red()));
        exp.push(pixel!(2,0, Color::red()));
        exp.push(pixel!(2,2, Color::red()));
        exp.push(pixel!(0,1, Color::red()));
        exp.push(pixel!(2,1, Color::red()));
        assert_eq!(exp, rect);

    }


}