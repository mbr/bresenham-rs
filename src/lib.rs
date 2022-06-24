//! Iterator-based Bresenham's line drawing algorithm
//!
//! [Bresenham's line drawing algorithm]
//! (https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) is fast
//! algorithm to draw a line between two points. This crate implements the fast
//! integer variant, using an iterator-based appraoch for flexibility. It
//! calculates coordinates without knowing anything about drawing methods or
//! surfaces.
//!
//! Example:
//!
//! ```rust
//! extern crate bresenham;
//! use bresenham::Bresenham;
//!
//! fn main() {
//!     for (x, y) in Bresenham::new((0, 1), (6, 4)) {
//!         println!("{}, {}", x, y);
//!     }
//! }
//! ```
//!
//! Will print:
//!
//! ```text
//! (0, 1)
//! (1, 1)
//! (2, 2)
//! (3, 2)
//! (4, 3)
//! (5, 3)
//! ```

#![no_std]

#[cfg(test)]
extern crate std;

use core::iter::Iterator;

/// Convenient typedef for two machines-sized integers
pub type Point = (isize, isize);

/// Line-drawing iterator
pub struct Bresenham {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    x1: isize,
    diff: isize,
    octant: Octant,
}

struct Octant(u8);

impl Octant {
    /// adapted from http://codereview.stackexchange.com/a/95551
    #[inline]
    fn from_points(start: Point, end: Point) -> Octant {
        let mut dx = end.0 - start.0;
        let mut dy = end.1 - start.1;

        let mut octant = 0;

        if dy < 0 {
            dx = -dx;
            dy = -dy;
            octant += 4;
        }

        if dx < 0 {
            let tmp = dx;
            dx = dy;
            dy = -tmp;
            octant += 2
        }

        if dx < dy {
            octant += 1
        }

        Octant(octant)
    }

    #[inline]
    fn to_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (p.1, -p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (-p.1, p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn from_octant0(&self, p: Point) -> Point {
        match self.0 {
            0 => (p.0, p.1),
            1 => (p.1, p.0),
            2 => (-p.1, p.0),
            3 => (-p.0, p.1),
            4 => (-p.0, -p.1),
            5 => (-p.1, -p.0),
            6 => (p.1, -p.0),
            7 => (p.0, -p.1),
            _ => unreachable!(),
        }
    }
}

impl Bresenham {
    /// Creates a new iterator.Yields intermediate points between `start`
    /// and `end`. Does include `start` but not `end`.
    #[inline]
    pub fn new(start: Point, end: Point) -> Bresenham {
        let octant = Octant::from_points(start, end);

        let start = octant.to_octant0(start);
        let end = octant.to_octant0(end);

        let dx = end.0 - start.0;
        let dy = end.1 - start.1;

        Bresenham {
            x: start.0,
            y: start.1,
            dx,
            dy,
            x1: end.0,
            diff: dy - dx,
            octant,
        }
    }
}

impl Iterator for Bresenham {
    type Item = Point;

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

        Some(self.octant.from_octant0(p))
    }
}


#[cfg(test)]
mod tests {
    use super::Bresenham;
    use std::vec::Vec;

    #[test]
    fn test_wp_example() {
        let bi = Bresenham::new((0, 1), (6, 4));
        let res: Vec<_> = bi.collect();

        assert_eq!(res, [(0, 1), (1, 1), (2, 2), (3, 2), (4, 3), (5, 3)])
    }

    #[test]
    fn test_inverse_wp() {
        let bi = Bresenham::new((6, 4), (0, 1));
        let res: Vec<_> = bi.collect();

        assert_eq!(res, [(6, 4), (5, 4), (4, 3), (3, 3), (2, 2), (1, 2)])
    }

    #[test]
    fn test_straight_hline() {
        let bi = Bresenham::new((2, 3), (5, 3));
        let res: Vec<_> = bi.collect();

        assert_eq!(res, [(2, 3), (3, 3), (4, 3)]);
    }

    #[test]
    fn test_straight_vline() {
        let bi = Bresenham::new((2, 3), (2, 6));
        let res: Vec<_> = bi.collect();

        assert_eq!(res, [(2, 3), (2, 4), (2, 5)]);
    }
}
