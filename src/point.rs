use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn normalized(&self) -> Point {
        let mag = self.magnitude();
        Point::new(self.x / mag, self.y / mag)
    }

    pub fn magnitude(&self) -> f64 {
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        (xx + yy).sqrt()
    }

    pub fn distance(&self, other: &Point) -> f64 {
        (*other - *self).magnitude()
    }

    pub fn rotate(&mut self, radians: f64) {
        self.x = self.x * radians.cos() - self.y * radians.sin();
        self.y = self.y * radians.sin() + self.y * radians.cos();
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<f64> for Point {
    type Output = Point;

    fn add(self, other: f64) -> Point {
        Point::new(self.x + other, self.y + other)
    }
}

impl Add<Point> for f64 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}


impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Sub<f64> for Point {
    type Output = Point;

    fn sub(self, other: f64) -> Point {
        Point::new(self.x - other, self.y - other)
    }
}

impl Sub<Point> for f64 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}


impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(self.x * other.x, self.y * other.y)
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = Point::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point::new(self.x * other, self.y * other)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        other * self
    }
}


impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point::new(self.x / other.x, self.y / other.y)
    }
}

impl DivAssign for Point {
    fn div_assign(&mut self, other: Point) {
        *self = Point::new(self.x / other.x, self.y / other.y)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point::new(self.x / other, self.y / other)
    }
}

impl Div<Point> for f64 {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        other / self
    }
}