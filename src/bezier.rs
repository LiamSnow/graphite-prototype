use crate::point::Point;

pub struct CubicBezier {
    /// anchor 1
    pub a1: Point,
    /// control 1
    pub c1: Point,
    /// control 2
    pub c2: Point,
    /// anchor 2
    pub a2: Point,
}

impl CubicBezier {
    pub fn new(a1: Point, c1: Point, c2: Point, a2: Point) -> Self {
        Self {
            a1,
            c1,
            c2,
            a2,
        }
    }

    pub fn set_index(&mut self, i: usize, val: Point) {
        match i {
            0 => self.a1 = val,
            1 => self.c1 = val,
            2 => self.c2 = val,
            3 => self.a2 = val,
            _ => panic!(),
        }
    }

    pub fn get_index(&self, i: usize) -> &Point {
        match i {
            0 => &self.a1,
            1 => &self.c1,
            2 => &self.c2,
            3 => &self.a2,
            _ => panic!(),
        }
    }

    pub fn to_path(&self) -> String {
        format!(
            "M {} {} c {} {} {} {} {} {}",
            self.a1.x,
            self.a1.y,
            self.c1.x - self.a1.x,
            self.c1.y - self.a1.y,
            self.c2.x - self.a1.x,
            self.c2.y - self.a1.y,
            self.a2.x - self.a1.x,
            self.a2.y - self.a1.y
        )
    }

    // creates two paths offset from this one
    pub fn split(&self) -> (CubicBezier, CubicBezier) {
        const DIST: f64 = 1.2;
        let n1 = (self.c1 - self.a1).normal();
        let n2 = (self.a2 - self.c2).normal();
        (
            CubicBezier {
                a1: self.a1.offset(&n1, -DIST),
                c1: self.c1.offset(&n1, -DIST),
                c2: self.c2.offset(&n2, -DIST),
                a2: self.a2.offset(&n2, -DIST),
            },
            CubicBezier {
                a1: self.a1.offset(&n1, DIST),
                c1: self.c1.offset(&n1, DIST),
                c2: self.c2.offset(&n2, DIST),
                a2: self.a2.offset(&n2, DIST),
            }
        )
    }
}
