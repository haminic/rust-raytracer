use crate::objects::base::*;
use crate::objects::instances::*;
use crate::prelude::*;

// Builder pattern for ease of use
// Usage:
// let sphere = HittableBuilder::new(Sphere::new(Point3::ZERO, 1.0, mat))
//     .translate(Vec3::ZERO)
//     .rotate_x(23.0)
//     .build();
pub struct HittableBuilder<T> {
    object: T,
}

impl<T: Hittable> HittableBuilder<T> {
    pub fn new(object: T) -> Self {
        Self { object }
    }

    pub fn build(self) -> T {
        self.object
    }

    pub fn translate(self, offset: Vec3) -> HittableBuilder<Translated<T>> {
        HittableBuilder::new(Translated::new(self.object, offset))
    }

    pub fn translating(self, offset: Vec3) -> HittableBuilder<Translating<T>> {
        HittableBuilder::new(Translating::new(
            self.object,
            Vec3::ZERO,
            offset,
        ))
    }

    pub fn rotate_x_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotated<T>> {
        HittableBuilder::new(Rotated::new(self.object, pivot, Axis::X, degrees))
    }

    pub fn rotate_y_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotated<T>> {
        HittableBuilder::new(Rotated::new(self.object, pivot, Axis::Y, degrees))
    }

    pub fn rotate_z_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotated<T>> {
        HittableBuilder::new(Rotated::new(self.object, pivot, Axis::Z, degrees))
    }

    pub fn rotate_x(self, degrees: f64) -> HittableBuilder<Rotated<T>> {
        self.rotate_x_about(Point3::ZERO, degrees)
    }

    pub fn rotate_y(self, degrees: f64) -> HittableBuilder<Rotated<T>> {
        self.rotate_y_about(Point3::ZERO, degrees)
    }

    pub fn rotate_z(self, degrees: f64) -> HittableBuilder<Rotated<T>> {
        self.rotate_z_about(Point3::ZERO, degrees)
    }

    pub fn rotating_x_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotating<T>> {
        HittableBuilder::new(Rotating::new(self.object, pivot, Axis::X, 0.0, degrees))
    }

    pub fn rotating_y_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotating<T>> {
        HittableBuilder::new(Rotating::new(self.object, pivot, Axis::Y, 0.0, degrees))
    }

    pub fn rotating_z_about(self, pivot: Point3, degrees: f64) -> HittableBuilder<Rotating<T>> {
        HittableBuilder::new(Rotating::new(self.object, pivot, Axis::Z, 0.0, degrees))
    }

    pub fn rotating_x(self, degrees: f64) -> HittableBuilder<Rotating<T>> {
        self.rotating_x_about(Point3::ZERO, degrees)
    }

    pub fn rotating_y(self, degrees: f64) -> HittableBuilder<Rotating<T>> {
        self.rotating_y_about(Point3::ZERO, degrees)
    }

    pub fn rotating_z(self, degrees: f64) -> HittableBuilder<Rotating<T>> {
        self.rotating_z_about(Point3::ZERO, degrees)
    }

    pub fn scale_about(self, pivot: Point3, scale: Vec3) -> HittableBuilder<Scaled<T>> {
        HittableBuilder::new(Scaled::new(self.object, scale, pivot))
    }

    pub fn scale(self, ratio: f64) -> HittableBuilder<Scaled<T>> {
        self.scale_about(Point3::ZERO, Vec3::new(ratio, ratio, ratio))
    }

    pub fn scale_x(self, s: f64) -> HittableBuilder<Scaled<T>> {
        self.scale_about(Point3::ZERO, Vec3::new(s, 1.0, 1.0))
    }

    pub fn scale_y(self, s: f64) -> HittableBuilder<Scaled<T>> {
        self.scale_about(Point3::ZERO, Vec3::new(1.0, s, 1.0))
    }

    pub fn scale_z(self, s: f64) -> HittableBuilder<Scaled<T>> {
        self.scale_about(Point3::ZERO, Vec3::new(1.0, 1.0, s))
    }

    pub fn scaling_about(self, start: Vec3, end: Vec3, pivot: Point3) -> HittableBuilder<Scaling<T>> {
        HittableBuilder::new(Scaling::new(self.object, start, end, pivot))
    }

    pub fn scaling(self, ratio: f64) -> HittableBuilder<Scaling<T>> {
        self.scaling_about(Vec3::ONE, Vec3::new(ratio, ratio, ratio), Point3::ZERO)
    }

    pub fn scaling_x(self, s: f64) -> HittableBuilder<Scaling<T>> {
        self.scaling_about(Vec3::ONE, Vec3::new(s, 1.0, 1.0), Point3::ZERO)
    }

    pub fn scaling_y(self, s: f64) -> HittableBuilder<Scaling<T>> {
        self.scaling_about(Vec3::ONE, Vec3::new(1.0, s, 1.0), Point3::ZERO)
    }

    pub fn scaling_z(self, s: f64) -> HittableBuilder<Scaling<T>> {
        self.scaling_about(Vec3::ONE, Vec3::new(1.0, 1.0, s), Point3::ZERO)
    }
    
}
