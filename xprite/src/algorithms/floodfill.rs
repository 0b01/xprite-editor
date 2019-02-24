use crate::prelude::*;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FloodFillDegrees {
    /// top bottom left right
    Four,
    /// you can fill a thin line using this
    Eight,
}

impl FloodFillDegrees {
    pub fn as_str(&self) -> &str {
        match self {
            FloodFillDegrees::Four => "Four",
            FloodFillDegrees::Eight => "Eight",
        }
    }

    pub const VARIANTS: [FloodFillDegrees; 2] = [FloodFillDegrees::Four, FloodFillDegrees::Eight];
}

impl FromStr for FloodFillDegrees {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Four" => Ok(FloodFillDegrees::Four),
            "Eight" => Ok(FloodFillDegrees::Eight),
            _ => Err(()),
        }
    }
}

impl Default for FloodFillDegrees {
    fn default() -> Self {
        FloodFillDegrees::Four
    }
}

/// flood fill algorithm
/// converts pixels into a grid of size (w, h)
/// The operation starts at origin
pub fn floodfill(w: f64, h: f64, pix: &Pixels, origin: Vec2f, bg_col: Option<Color>, color: Color, degrees: FloodFillDegrees) -> Pixels {
    if oob(origin.x, origin.y, w, h) {
        return Pixels::new();
    }
    let mut ret = Pixels::new();
    let canvas = pix.as_mat(w as usize, h as usize);
    let mut stack = vec![origin];

    let mut visited = vec![vec![false; w as usize]; h as usize];
    let mut neighbors = Vec::with_capacity(match degrees {
        FloodFillDegrees::Four => 4,
        FloodFillDegrees::Eight => 8,
    });
    while let Some(point) = stack.pop() {
        let Vec2f { x, y } = point;
        match (bg_col, canvas[y as usize][x as usize]) {
            (Some(bg), Some(Pixel { color, .. })) => {
                if bg != color {
                    continue;
                }
            }
            (None, Some(_)) => continue,
            (Some(_), None) => continue,
            (None, None) => (),
        };
        neighbors.clear(); // Checking only 4 neighbors

        if x < w - 1. {
            neighbors.push((x + 1., y))
        };
        if x > 0. {
            neighbors.push((x - 1., y))
        };
        if y < h - 1. {
            neighbors.push((x, y + 1.))
        };
        if y > 0. {
            neighbors.push((x, y - 1.))
        };
        if degrees == FloodFillDegrees::Eight {
            if x < w - 1. && y < h - 1. {
                neighbors.push((x + 1., y + 1.))
            };
            if x > 0. && y > 0. {
                neighbors.push((x - 1., y - 1.))
            };
            if y < h - 1. && x > 0. {
                neighbors.push((x - 1., y + 1.))
            };
            if y > 0. && x < w - 1. {
                neighbors.push((x + 1., y - 1.))
            };
        }

        for &(nx, ny) in &neighbors {
            if visited[ny as usize][nx as usize] {
                continue;
            };
            stack.push(Vec2f { x: nx, y: ny });
            visited[ny as usize][nx as usize] = true;
        }
        ret.push(Pixel { point, color });
        visited[y as usize][x as usize] = true;
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_floodfill() {
        // xx
        // oo
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0., 0., Color::black()));
        pixs.push(pixel!(0., 1., Color::black()));
        let to_fill = floodfill(2., 2., &pixs, Vec2f { x: 1., y: 1. }, None, Color::red(), FloodFillDegrees::Four);
        assert_eq!(Pixels::from_slice(&vec![pixel!(1, 1, Color::red()), pixel!(1, 0, Color::red()),]), to_fill);
    }

    #[test]
    fn test_floodfill2() {
        //  x
        // xox
        //  x
        let mut pixs = Pixels::new();
        pixs.push(pixel!(1., 0., Color::black()));
        pixs.push(pixel!(0., 1., Color::black()));
        pixs.push(pixel!(1., 2., Color::black()));
        pixs.push(pixel!(2., 1., Color::black()));

        let to_fill = floodfill(100., 100., &pixs, Vec2f { x: 1., y: 1. }, None, Color::black(), FloodFillDegrees::Four);
        assert_eq!(Pixels::from_slice(&vec![pixel!(1, 1, Color::black()),]), to_fill);

        let to_fill = floodfill(100., 100., &pixs, Vec2f { x: 1., y: 1. }, None, Color::black(), FloodFillDegrees::Four);
        assert_eq!(Pixels::from_slice(&vec![pixel!(1, 1, Color::black()),]), to_fill);
    }

    #[test]
    fn test_floodfill3() {
        //  xx
        // xoox
        //  xx
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0., 1., Color::black()));
        pixs.push(pixel!(0., 2., Color::black()));
        pixs.push(pixel!(1., 0., Color::black()));
        pixs.push(pixel!(1., 3., Color::black()));
        pixs.push(pixel!(2., 1., Color::black()));
        pixs.push(pixel!(2., 2., Color::black()));

        let to_fill = floodfill(4., 3., &pixs, Vec2f { x: 1., y: 1. }, None, Color::blue(), FloodFillDegrees::Four);
        assert_eq!(Pixels::from_slice(&vec![pixel!(1, 1, Color::blue()), pixel!(1, 2, Color::blue()),]), to_fill);
    }

    #[test]
    fn test_floodfill_8way() {
        // fill the thin outline (x)

        //  xx
        // xoox
        //  xx
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0., 1., Color::black()));
        pixs.push(pixel!(0., 2., Color::black()));
        pixs.push(pixel!(1., 0., Color::black()));
        pixs.push(pixel!(1., 3., Color::black()));
        pixs.push(pixel!(2., 1., Color::black()));
        pixs.push(pixel!(2., 2., Color::black()));

        let to_fill = floodfill(
            4.,
            3.,
            &pixs,
            Vec2f { x: 0., y: 1. },
            Some(Color::black()),
            Color::blue(),
            FloodFillDegrees::Eight,
        );
        assert_eq!(
            Pixels::from_slice(&vec![
                pixel!(0., 1., Color::blue()),
                pixel!(0., 2., Color::blue()),
                pixel!(1., 0., Color::blue()),
                pixel!(1., 3., Color::blue()),
                pixel!(2., 1., Color::blue()),
                pixel!(2., 2., Color::blue())
            ]),
            to_fill
        );
    }
}
