use crate::prelude::*;
use crate::objects::{Aabb, Hit, Hittable};

pub struct Translated {
    object: Box<dyn Hittable>,
    offset: Lerp<Vec3>,
    bbox: Aabb,
}

impl Translated {
    pub fn new(object: impl Hittable + 'static, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object: Box::new(object),
            offset: Lerp::new(offset, offset),
            bbox,
        }
    }

    // Moves object from from offset1 to offset2 within shutter time
    pub fn new_moving(object: impl Hittable + 'static, offset1: Vec3, offset2: Vec3) -> Self {
        let bbox1 = object.bounding_box() + offset1;
        let bbox2 = object.bounding_box() + offset2;
        Self {
            object: Box::new(object),
            offset: Lerp::new(offset1, offset2),
            bbox: Aabb::enclosing(bbox1, bbox2),
        }
    }
}

impl Hittable for Translated {
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
