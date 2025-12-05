use super::{Hit, Material, Scatter};
use crate::prelude::*;

pub struct Dielectric {
    ior_ratio: f64,
}

impl Dielectric {
    pub fn new(ior_ratio: f64) -> Self {
        Self { ior_ratio }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ior_ratio: f64 = if hit.front_face {
            1.0 / self.ior_ratio
        } else {
            self.ior_ratio
        };

        let unit_direction = ray_in.direction.unit_vector();
        let cos_theta = hit.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ior_ratio * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, ior_ratio) > random_f64() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, ior_ratio)
        };

        Some(Scatter {
            ray_out: Ray::new(hit.point, direction),
            attenuation,
        })
    }
}

fn reflectance(cos: f64, ior_ratio: f64) -> f64 {
    // Schlick's approximation
    let mut r0 = (1.0 - ior_ratio) / (1.0 + ior_ratio);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
