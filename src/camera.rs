use crate::objects::hittable::{HitRecord, Hittable};
use crate::prelude::*;

pub struct Camera {
    aspect_ratio: f64,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        // TODO
        unimplemented!()
    }

    pub fn render(&self, writer: &mut BufWriter<File>, world: &dyn Hittable) {
        // TODO
        unimplemented!()
    }

    fn initialize(&mut self) {
        // TODO
        unimplemented!()
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // TODO
        unimplemented!()
    }

    fn sample_square() -> Vec3 {
        // TODO
        unimplemented!()
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) {
        // TODO
        unimplemented!()
    }
}
