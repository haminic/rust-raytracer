use crate::prelude::*;
use crate::{objects::{Aabb, Hit, Hittable}, materials::Material};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
            bbox: Aabb::new(
                Interval::centered_at(center.x, radius),
                Interval::centered_at(center.y, radius),
                Interval::centered_at(center.z, radius),
            ),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        /*
            ray(t) = Q+t*d ; d = direction of r
            (C-(Q+t*d))(C-(Q+t*d)) = r^2 -> find solution t
            [d*d] t^2 - [2d.dot(C-Q)] * t + [(C-Q)(C-Q) - r^2] = 0

            use h = -b/2 = d.dot(C-Q)
            solution root = ( h +- sqrt( h*h - a*c ) ) / a

        */

        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

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
        let outward_normal = (point - self.center) / self.radius;
        let hit = Hit::new(&ray, point, outward_normal, self.mat.clone(), root);
        Some(hit)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
