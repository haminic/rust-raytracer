use super::{HitRecord, Material, Scatter};
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
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        Some(Scatter {
            ray_out: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}
