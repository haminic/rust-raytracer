use super::{Hit, Material, Scatter};
use crate::prelude::*;

pub struct Metal {
    albedo: Color,    
}

impl Metal {
    pub const fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected: Vec3 = ray_in.direction.reflect(hit.normal);
        Some(Scatter {
            ray_out: Ray::with_time(hit.point, reflected, ray_in.time),
            attenuation: self.albedo,
        })
    }
}