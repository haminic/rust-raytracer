mod dielectric;
mod lambertian;
mod light;
mod metal;
mod isotropic;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use light::DiffuseLight;
pub use metal::Metal;
pub use isotropic::Isotropic;

use crate::prelude::*;
use crate::objects::Hit;

pub trait Material: Send + Sync {

    fn scatter(&self, _ray_in: &Ray, _hit: &Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self, _point: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Scatter {
    pub ray_out: Ray,
    pub attenuation: Color,
}
