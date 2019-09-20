use crate::prelude::*;

// TODO: eliminate clone
pub fn pixel_perfect(path: &mut Pixels) {
    if path.len() == 1 || path.is_empty() {
        return;
    }
    let mut take = vec![false; path.len()];
    let mut c = 0;
    while c < path.len() {
        macro_rules! prev {
            () => {
                path[c - 1].point
            };
        }
        macro_rules! next {
            () => {
                path[c + 1].point
            };
        }
        macro_rules! curr {
            () => {
                path[c].point
            };
        }
        if c > 0
            && c + 1 < path.len()
            && (prev!().x == curr!().x || prev!().y == curr!().y)
            && (next!().x == curr!().x || next!().y == curr!().y)
            && prev!().x != next!().x
            && prev!().y != next!().y
        {
            c += 1;
        }
        take[c] = true;
        c += 1;
    }
    let mut idx = 0;
    path.0.retain(|_| {
        idx += 1;
        take[idx - 1]
    });
}

pub fn pixel_antiperfect(path: &Pixels) -> Pixels {
    if path.len() == 1 || path.is_empty() {
        return path.clone();
    }
    let mut ret = Pixels::new();

    for (curr, next) in path.iter().zip(path.iter().skip(1)) {
        if curr.point.x != next.point.x && curr.point.y != next.point.y {
            let mut between = *curr;
            match (next.point.x > curr.point.x, next.point.y > curr.point.y) {
                (true, true) => {
                    between.point.x += 1.;
                }
                (true, false) => {
                    between.point.x += 1.;
                }
                (false, true) => {
                    between.point.x -= 1.;
                }
                (false, false) => {
                    between.point.y -= 1.;
                }
            };

            ret.push(*curr);
            ret.push(between);
            ret.push(*next);
        } else {
            ret.push(*curr);
            ret.push(*next);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pp() {
        let mut path = pixels!(pixel!(0., 0., Color::red()), pixel!(0., 1., Color::red()), pixel!(1., 1., Color::red()));

        pixel_perfect(&mut path);
        assert_eq!(path, pixels!(pixel!(0., 0., Color::red()), pixel!(1., 1., Color::red())));
    }

    #[test]
    fn test_antipp() {
        assert_eq!(
            pixel_antiperfect(&pixels!(pixel!(0., 0., Color::red()), pixel!(1., 1., Color::red()))),
            pixels!(pixel!(0., 0., Color::red()), pixel!(0., 1., Color::red()), pixel!(1., 1., Color::red()))
        );
    }
}
