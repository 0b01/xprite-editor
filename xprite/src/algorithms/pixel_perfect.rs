use crate::prelude::*;

// TODO: eliminate clone
pub fn pixel_perfect(path: &Pixels) -> Pixels {
    if path.len() == 1 || path.len() == 0 {
        return path.clone();
    }
    let mut ret = Pixels::new();
    let mut c = 0;
    while c < path.len() {
        macro_rules! prev { () => { path[c-1].point } }
        macro_rules! next { () => { path[c + 1].point } }
        macro_rules! curr { () => { path[c].point } }
        if c > 0
            && c + 1 < path.len()
            && (prev!().x == curr!().x || prev!().y == curr!().y)
            && (next!().x == curr!().x || next!().y == curr!().y)
            && prev!().x != next!().x
            && prev!().y != next!().y
        {
            c += 1;
        }
        ret.push(path[c]);
        c += 1;
    }
    ret
}

pub fn pixel_antiperfect(path: &Pixels) -> Pixels {
    if path.len() == 1 || path.len() == 0 {
        return path.clone();
    }
    let mut ret = Pixels::new();

    for (curr, next) in path.iter().zip(path.iter().skip(1)) {
        dbg!((curr, next));
        if curr.point.x != next.point.x
        && curr.point.y != next.point.y {
            let mut between = curr.clone();
            between.point.x += 1.;
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
        let path = pixels!(
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
            pixel!(1., 1., Color::red())
        );

        let ret = pixel_perfect(&path);
        assert_eq!(ret, pixels!(
            pixel!(0., 0., Color::red()),
            pixel!(1., 1., Color::red())
        ));
    }

    #[test]
    fn test_antipp() {
        let path = pixels!(
            pixel!(0., 0., Color::red()),
            pixel!(1., 1., Color::red())
        );
        let ret = pixel_antiperfect(&path);
        assert_eq!(ret, pixels!(
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
            pixel!(1., 1., Color::red())
        ));
    }
}
