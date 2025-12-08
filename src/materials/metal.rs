use super::{Material, Scatter};
use crate::objects::Hit;
use crate::prelude::*;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub const SILVER_ALBEDO: Color = Color::new(252.0 / 256.0, 250.0 / 256.0, 245.0 / 256.0);
    pub const GOLD_ALBEDO: Color = Color::new(255.0 / 256.0, 226.0 / 256.0, 155.0 / 256.0);
    pub const CHROME_ALBEDO: Color = Color::new(196.0 / 255.0, 197.0 / 255.0, 197.0 / 255.0);

    pub const fn new(albedo: Color) -> Self {
        Self { albedo, fuzz: 0.0 }
    }

    pub const fn with_fuzz(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: (Interval::UNIT.clamp(fuzz)),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut reflected: Vec3 = ray_in.direction.reflect(hit.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        Some(Scatter {
            ray_out: Ray::with_time(hit.point, reflected, ray_in.time),
            attenuation: self.albedo,
        })
    }
}
