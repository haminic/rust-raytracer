use crate::prelude::*;
use crate::materials::{Isotropic, Material};
use crate::objects::{Hittable, Aabb, Hit};

static EPSILON: f64 = 0.0001;

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
    bbox: Aabb,
}

impl ConstantMedium {
    pub fn new(boundary: impl Hittable + 'static, density: f64, albedo: Color) -> Self{
        Self { 
            neg_inv_density: (-1.0 / density),  
            phase_function: Arc::new(Isotropic::new(albedo)),
            bbox: boundary.bounding_box(),
            boundary: Box::new(boundary), 
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {

        let Some(mut hit1) = self.boundary.hit(ray, Interval::UNIVERSE) else { return None; };
        let Some(mut hit2) = self.boundary.hit(ray, Interval::new(hit1.t + EPSILON, INFINITY)) else { return None; };

        if hit1.t < t_range.min {hit1.t = t_range.min;}
        if hit2.t > t_range.max {hit2.t = t_range.max;}

        if hit1.t >= hit2.t { return None; }
        if hit1.t < 0.0 { hit1.t = 0.0; }

        let ray_lenght:f64 = ray.direction.length();
        // I found it here typo -> total reflection -> kinetice theory of gas is crying
        let distance_inside_boundary:f64 = (hit2.t-hit1.t) * ray_lenght;
        let hit_distance:f64 = self.neg_inv_density * random_unit_f64().ln();

        // forgot to return HERE, GENPHYS is crying now
        if hit_distance > distance_inside_boundary { return None; }

        let t: f64 = hit1.t + hit_distance / ray_lenght;

        // dont care normal, front_face
        Some(Hit::new(
            ray,
            ray.at(t),
            Vec3::new(1.0,0.0,0.0), 
            self.phase_function.clone(),
            t,
        ))
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}