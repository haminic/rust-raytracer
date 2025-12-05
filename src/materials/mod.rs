pub mod lambertian;

use crate::objects::HitRecord;
use crate::prelude::*;

pub use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
