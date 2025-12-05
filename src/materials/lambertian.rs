use super::{Hit, Material, Scatter};
use crate::prelude::*;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        Some(Scatter {
            ray_out: Ray::new(hit.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
