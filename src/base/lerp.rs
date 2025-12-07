use std::ops::{Add, Mul};

pub struct Lerp<T> {
    pub begin: T,
    pub end: T,
}

impl<T> Lerp<T>
where
    T: Mul<f64, Output = T> + Add<Output = T> + Copy,
{
    pub fn new(begin: T, end: T) -> Self {
        Self { begin, end }
    }

    pub fn at(&self, t: f64) -> T {
        self.begin * (1.0 - t) + self.end * t
    }
}
