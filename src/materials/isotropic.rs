use crate::materials::{Material, Scatter};
use crate::objects::Hit;
use crate::prelude::*;

/*
    This is not actally isotropic
    It is temporary fix to running Volumetric mass
    Assume: texture at (u,v) return albedo
*/

pub struct Isotropic {
    albedo: Color,
}

impl Isotropic {
    pub fn new(albedo: Color) -> Arc<dyn Material> {
        Arc::new(Self { albedo })
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let scattered = Ray::with_time(hit.point, Vec3::random_unit_vector(), ray_in.time);
        Some(Scatter {
            ray_out: scattered,
            attenuation: self.albedo,
        })
    }
}
