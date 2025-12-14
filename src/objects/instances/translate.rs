use crate::objects::base::*;
use crate::prelude::*;

pub struct Translated<T> {
    object: T,
    offset: Vec3,
    bbox: Aabb,
}

impl<T: Hittable> Translated<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<T: Hittable> Hittable for Translated<T> {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let translated_ray = Ray::with_time(ray.origin - self.offset, ray.direction, ray.time);
        self.object.hit(&translated_ray, t_range).map(|mut hit| {
            hit.point += self.offset;
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Translating<T> {
    object: T,
    offset: Lerp<Vec3>,
    bbox: Aabb,
}

impl<T: Hittable> Translating<T> {
    // Moves object from from offset1 to offset2 within shutter time
    pub fn new(object: T, offset1: Vec3, offset2: Vec3) -> Self {
        let bbox1 = object.bounding_box() + offset1;
        let bbox2 = object.bounding_box() + offset2;
        Self {
            object,
            offset: Lerp::new(offset1, offset2),
            bbox: Aabb::enclosing(bbox1, bbox2),
        }
    }
}

impl<T: Hittable> Hittable for Translating<T> {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let offset = self.offset.at(ray.time);
        let translated_ray = Ray::with_time(ray.origin - offset, ray.direction, ray.time);
        self.object.hit(&translated_ray, t_range).map(|mut hit| {
            hit.point += offset;
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
