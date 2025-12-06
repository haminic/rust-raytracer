mod dielectric;
mod lambertian;
mod metal;

use crate::objects::Hit;
use crate::prelude::*;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
