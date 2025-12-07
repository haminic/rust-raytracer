use std::ops::Add;

use crate::prelude::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
    pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);

    pub const fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub const fn enclosing(a: Self, b: Self) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub const fn centered_at(center: f64, radius: f64) -> Self {
        Self::new(center - radius, center + radius)
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub const fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;
    fn add(self, rhs: Interval) -> Self::Output {
        Interval::new(rhs.min + self, rhs.max + self)
    }
}
