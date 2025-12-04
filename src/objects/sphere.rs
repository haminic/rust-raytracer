use crate::objects::prelude::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        // let b = -2.0 * ray.direction.dot(oc);
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        // let discriminant = b * b - 4.0 * a * c;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !t_range.surrounds(root) {
            root = (h + sqrtd) / a;
            if !t_range.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(&ray, outward_normal);

        true
    }
}
