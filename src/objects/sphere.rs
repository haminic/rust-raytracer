use super::{Hit, Hittable, Material};
use crate::prelude::*;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat,
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1),
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        // let b = -2.0 * ray.direction.dot(oc);
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        // let discriminant = b * b - 4.0 * a * c;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !t_range.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_range.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - current_center) / self.radius;
        let hit = Hit::new(&ray, point, outward_normal, self.mat.clone(), root);
        Some(hit)
    }
}
