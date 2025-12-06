use crate::prelude::*;

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

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }

    pub fn enclosing(a: Self, b: Self) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub fn centered_at(center: f64, radius: f64) -> Self {
        Self::new(center - radius, center + radius)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }
}
