use crate::prelude::*;

pub fn find_perimeter(w: usize, h: usize, pixs: &Pixels) -> Pixels {
    let mut ret = Pixels::new();
    let grid = pixs.as_bool_arr(w, h);
    let canvas = pixs.as_arr(w, h);
    for row in 0..w {
        for col in 0..h {
            if grid[row][col] {
                if (row >= 1 && grid[row - 1][col])
                && (row + 1 < h && grid[row + 1][col])
                && (col >= 1 && grid[row][col - 1])
                && (col + 1 < w && grid[row][col + 1]) {
                    continue
                }
                else {
                    ret.push(canvas[row][col].unwrap())
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perimeter() {
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0,0,Color::red()));
        pixs.push(pixel!(0,1,Color::red()));
        pixs.push(pixel!(0,2,Color::red()));
        pixs.push(pixel!(1,0,Color::red()));
        pixs.push(pixel!(1,1,Color::red()));
        pixs.push(pixel!(1,2,Color::red()));
        pixs.push(pixel!(2,0,Color::red()));
        pixs.push(pixel!(2,1,Color::red()));
        pixs.push(pixel!(2,2,Color::red()));

        let mut result = Pixels::new();
        result.push(pixel!(0,0,Color::red()));
        result.push(pixel!(0,1,Color::red()));
        result.push(pixel!(0,2,Color::red()));
        result.push(pixel!(1,0,Color::red()));
        result.push(pixel!(1,2,Color::red()));
        result.push(pixel!(2,0,Color::red()));
        result.push(pixel!(2,1,Color::red()));
        result.push(pixel!(2,2,Color::red()));

        let peri = find_perimeter(3, 3, &pixs);
        assert_eq!(result, peri);
    }
}