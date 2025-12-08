use crate::prelude::*;
use crate::objects::Hit;
use super::{Material, Scatter};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        Some(Scatter {
            ray_out: Ray::with_time(hit.point, scatter_direction, ray_in.time),
            attenuation: self.albedo,
        })
    }
}
