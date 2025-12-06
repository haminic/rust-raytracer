use super::{Material};
use crate::prelude::*;

/*
    This is class is equivalence to hit record
*/
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
            // ray is outside the sphere
            outward_normal
        } else {
            // ray is inside the sphere
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