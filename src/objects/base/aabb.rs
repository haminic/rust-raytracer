use std::ops::{Add, Mul, Sub};

use crate::prelude::*;

/*
    Axis-Aligned Bounding Box
    Use for Bvh optimizing
*/

#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const EMPTY: Self = Self::new(Interval::EMPTY, Interval::EMPTY, Interval::EMPTY);

    pub const fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimum();
        aabb
    }

    pub const fn from_corners(a: Point3, b: Point3) -> Self {
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        Self::new(x, y, z)
    }

    pub const fn enclosing(a: Self, b: Self) -> Self {
        Self::new(
            Interval::enclosing(a.x, b.x),
            Interval::enclosing(a.y, b.y),
            Interval::enclosing(a.z, b.z),
        )
    }

    pub const fn axis(&self, axis: Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: Interval) -> bool {
        let mut lower_bound = t_range.min;
        let mut upper_bound = t_range.max;

        for axis in Axis::AXES {
            let interval = self.axis(axis);
            let adinv = 1.0 / ray.direction.axis(axis);

            let t0 = (interval.min - ray.origin.axis(axis)) * adinv;
            let t1 = (interval.max - ray.origin.axis(axis)) * adinv;

            if t0 < t1 {
                lower_bound = lower_bound.max(t0);
                upper_bound = upper_bound.min(t1);
            } else {
                lower_bound = lower_bound.max(t1);
                upper_bound = upper_bound.min(t0);
            }

            if upper_bound <= lower_bound {
                return false;
            }
        }
        true
    }

    const fn pad_to_minimum(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;
    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;
    fn add(self, rhs: Aabb) -> Self::Output {
        Aabb::new(rhs.x + self.x, rhs.y + self.y, rhs.z + self.z)
    }
}

impl Sub<Vec3> for Aabb {
    type Output = Aabb;
    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Aabb> for Vec3 {
    type Output = Aabb;
    fn sub(self, rhs: Aabb) -> Self::Output {
        rhs + (-self)
    }
}

impl Mul<Vec3> for Aabb {
    type Output = Aabb;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Aabb::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<Aabb> for Vec3 {
    type Output = Aabb;
    fn mul(self, rhs: Aabb) -> Self::Output {
        rhs * self
    }
}