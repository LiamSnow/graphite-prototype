use crate::point::Point;

pub struct CubicBezier {
    pub anchor1: Point,
    pub control1: Point,
    pub control2: Point,
    pub anchor2: Point,
}

impl CubicBezier {
    pub fn new(anchor1: Point, control1: Point, control2: Point, anchor2: Point) -> Self {
        Self {
            anchor1,
            control1,
            control2,
            anchor2,
        }
    }

    pub fn set_index(&mut self, i: usize, val: Point) {
        match i {
            0 => self.anchor1 = val,
            1 => self.control1 = val,
            2 => self.control2 = val,
            3 => self.anchor2 = val,
            _ => panic!(),
        }
    }

    pub fn get_index(&self, i: usize) -> &Point {
        match i {
            0 => &self.anchor1,
            1 => &self.control1,
            2 => &self.control2,
            3 => &self.anchor2,
            _ => panic!(),
        }
    }

    pub fn to_path(&self) -> String {
        format!(
            "M {} {} c {} {} {} {} {} {}",
            self.anchor1.x,
            self.anchor1.y,
            self.control1.x - self.anchor1.x,
            self.control1.y - self.anchor1.y,
            self.control2.x - self.anchor1.x,
            self.control2.y - self.anchor1.y,
            self.anchor2.x - self.anchor1.x,
            self.anchor2.y - self.anchor1.y
        )
    }
}
