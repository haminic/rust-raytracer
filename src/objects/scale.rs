use crate::objects::{Aabb, Hit, Hittable};
use crate::prelude::*;

pub struct Scaled<T> {
    object: T,
    scale: Vec3,
    center: Point3,
    bbox: Aabb,
}

impl<T: Hittable> Scaled<T> {
    pub fn new(object: T, scale: Vec3, center: Point3) -> Self {
        let bbox = scale * (object.bounding_box() - center) + center;
        Self {
            object,
            scale,
            center,
            bbox,
        }
    }
}

impl<T:Hittable> Hittable for Scaled<T> {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let scaled_ray = Ray::with_time((ray.origin - self.center) / self.scale + self.center, ray.direction / self.scale, ray.time);
        self.object.hit(&scaled_ray, t_range).map(|mut hit| {
            hit.point = self.scale * (hit.point - self.center) + self.center;
            hit.normal = (hit.normal / self.scale).unit_vector();
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Scaling<T> {
    object: T,
    scale: Lerp<Vec3>,
    center: Vec3,
    bbox: Aabb,
}

impl<T: Hittable> Scaling<T> {
    pub fn new(object: T, scale1: Vec3, scale2: Vec3, center: Vec3) -> Self {
        let bbox1 = scale1 * (object.bounding_box() - center) + center;
        let bbox2 = scale2 * (object.bounding_box() - center) + center;
        Self {
            object,
            scale: Lerp::new(scale1, scale2),
            center,
            bbox: Aabb::enclosing(bbox1, bbox2)
        }
    }
}

impl<T: Hittable> Hittable for Scaling<T> {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let scale = self.scale.at(ray.time);
        let scaled_ray = Ray::with_time((ray.origin - self.center) / scale + self.center, ray.direction / scale, ray.time);
        self.object.hit(&scaled_ray, t_range).map(|mut hit| {
            hit.point = scale * (hit.point - self.center) + self.center;
            hit.normal = (hit.normal / scale).unit_vector();
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}