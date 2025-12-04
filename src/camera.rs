use crate::objects::hittable::{HitRecord, Hittable};
use crate::prelude::*;

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
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

    fn ray_color(ray: &Ray, world: &dyn Hittable) {
        // TODO
        unimplemented!()
    }
}
