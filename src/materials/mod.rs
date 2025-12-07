mod dielectric;
mod lambertian;
mod metal;
mod light;

use crate::objects::Hit;
use crate::prelude::*;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use light::DiffuseLight;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> { None }
    fn emitted(&self, p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
