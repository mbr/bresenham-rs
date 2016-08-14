extern crate core;

use core::iter::Iterator;


struct Bresenham {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    x1: isize,
    diff: isize,
}

impl Bresenham {
    #[inline]
    fn new(start: (isize, isize), end: (isize, isize)) -> Bresenham {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        Bresenham {
            x: start.0,
            y: start.1,
            dx: dx,
            dy: dy,
            x1: end.0,
            diff: dy - dx,
        }
    }
}

impl Iterator for Bresenham {
    type Item = (isize, isize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x1 {
            return None;
        }

        let p = (self.x, self.y);

        if self.diff >= 0 {
            self.y += 1;
            self.diff -= self.dx;
        }

        self.diff += self.dy;

        // loop inc
        self.x += 1;

        Some(p)
    }
}


#[cfg(test)]
mod tests {
    use super::Bresenham;

    #[test]
    fn test_wp_example() {
        let bi = Bresenham::new((0, 1), (6, 4));
        let res: Vec<_> = bi.collect();

        assert_eq!(res, [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)])
    }
}
