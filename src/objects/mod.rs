mod hittable_list;
mod sphere;

pub use hittable_list::HittableList;
pub use sphere::Sphere;

use crate::{materials::Material, prelude::*};
use std::sync::Arc;

pub struct Hit {
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Hit {
    pub fn new(
        ray: &Ray,
        point: Point3,
        outward_normal: Vec3,
        mat: Arc<dyn Material>,
        t: f64,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            mat,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
}
