use crate::prelude::*;
use crate::objects::{Aabb, Hit, Hittable};

pub struct Translated {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translated {
    pub fn new(object: impl Hittable + 'static, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object: Box::new(object),
            offset,
            bbox,
        }
    }
}

impl Hittable for Translated {
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
