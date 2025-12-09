use crate::objects::{Aabb, Hit};
use crate::prelude::*;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}

pub fn to_hittable(object: impl Hittable + 'static) -> Box<dyn Hittable> {
    Box::new(object)
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        (**self).hit(&ray, t_range)
    }

    fn bounding_box(&self) -> Aabb {
        (**self).bounding_box()
    }
}
