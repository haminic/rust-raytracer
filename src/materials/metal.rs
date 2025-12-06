use super::{Hit, Material, Scatter};
use crate::prelude::*;

pub struct Metal {
    albedo: Color,    
}

impl Metal {
    pub const SILVER_ALBEDO: Color = Color::new(252.0/256.0, 250.0/256.0, 245.0/256.0);
    pub const GOLD_ALBEDO: Color = Color::new(255.0/256.0, 226.0/256.0, 155.0/256.0);
    pub const CHROME_ALBEDO: Color = Color::new(196.0/255.0, 197.0/255.0, 197.0/255.0);

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