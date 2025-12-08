use crate::materials::Material;
use crate::objects::{Aabb, Hit, Hittable};
use crate::prelude::*;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    d: f64,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::from_corners(q, q + u + v);
        let bbox_diagonal2 = Aabb::from_corners(q + u, q + v);
        let normal = u.cross(v).unit_vector();
        let d = normal.dot(q);
        Self {
            q,
            u,
            v,
            normal,
            d,
            mat,
            bbox: Aabb::enclosing(bbox_diagonal1, bbox_diagonal2),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-8_f64 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin)) / denom;
        if !t_range.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let vec = intersection - self.q;
        let u_length = vec.dot(self.u);
        let v_length = vec.dot(self.v);

        if Interval::new(0.0, self.u.length_squared()).contains(u_length)
            && Interval::new(0.0, self.v.length_squared()).contains(v_length)
        {
            Some(Hit::new(
                ray,
                intersection,
                self.normal,
                self.mat.clone(),
                t,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
