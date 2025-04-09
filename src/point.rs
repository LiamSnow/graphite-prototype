use std::ops::Sub;


#[derive(Default, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn normal(&self) -> Self {
        Point::new(-self.y, self.x).normalize()
    }

    pub fn normalize(&self) -> Self {
        let len  = (self.x * self.x + self.y * self.y).sqrt();
        if len > 0. {
            Point::new(self.x / len, self.y / len)
        }
        else {
            Point::default()
        }
    }

    pub fn offset(&self, dir: &Point, distance: f64) -> Point {
        Point {
            x: self.x + dir.x * distance,
            y: self.y + dir.y * distance,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}
