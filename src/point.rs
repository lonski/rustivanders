use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    pub fn to_ituple(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    pub fn to_utuple(&self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
