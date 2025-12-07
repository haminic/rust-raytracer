mod dielectric;
mod lambertian;
mod light;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use light::DiffuseLight;
pub use metal::Metal;

use crate::prelude::*;
use crate::objects::Hit;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self, p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
