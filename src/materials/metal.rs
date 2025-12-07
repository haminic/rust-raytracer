use crate::prelude::*;
use crate::objects::Hit;
use super::{Material, Scatter};

pub struct Metal {
    albedo: Color,    
}

impl Metal {
    #[allow(dead_code)]
    pub const SILVER_ALBEDO: Color = Color::new(252.0/256.0, 250.0/256.0, 245.0/256.0);
    #[allow(dead_code)]
    pub const GOLD_ALBEDO: Color = Color::new(255.0/256.0, 226.0/256.0, 155.0/256.0);
    #[allow(dead_code)]
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