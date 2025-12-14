use crate::objects::base::*;
use crate::objects::instances::*;
use crate::prelude::*;

// Builder pattern for ease of use
// Usage:
// let sphere = HittableBuilder::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0, mat))
//     .translate(Vec3::new(0.0, 0.0, 0.0))
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
            Vec3::new(0.0, 0.0, 0.0),
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
        self.rotate_x_about(Point3::new(0.0, 0.0, 0.0), degrees)
    }

    pub fn rotate_y(self, degrees: f64) -> HittableBuilder<Rotated<T>> {
        self.rotate_y_about(Point3::new(0.0, 0.0, 0.0), degrees)
    }

    pub fn rotate_z(self, degrees: f64) -> HittableBuilder<Rotated<T>> {
        self.rotate_z_about(Point3::new(0.0, 0.0, 0.0), degrees)
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
        self.rotating_x_about(Point3::new(0.0, 0.0, 0.0), degrees)
    }

    pub fn rotating_y(self, degrees: f64) -> HittableBuilder<Rotating<T>> {
        self.rotating_y_about(Point3::new(0.0, 0.0, 0.0), degrees)
    }

    pub fn rotating_z(self, degrees: f64) -> HittableBuilder<Rotating<T>> {
        self.rotating_z_about(Point3::new(0.0, 0.0, 0.0), degrees)
    }
}
