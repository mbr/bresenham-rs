extern crate core;

use core::iter::Iterator;

fn line(x0: isize, y0: isize, x1: isize, y1: isize) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut diff = dy - dx;

    let mut y = y0;

    for x in x0..x1 {
        println!("{},{}", x, y);

        if diff >= 0 {
            y = y + 1;
            diff = diff - dx;
        }
        diff = diff + dy;
    }
    println!("DONE");
}


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
    use super::{Bresenham, line};
    #[test]
    fn it_works() {
        line(0, 1, 6, 4);

        for p in Bresenham::new((0, 1), (6, 4)) {
            println!("{},{}", p.0, p.1);
        }
    }
}
