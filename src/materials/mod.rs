mod dielectric;
mod lambertian;

use crate::objects::Hit;
use crate::prelude::*;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
