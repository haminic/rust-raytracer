use super::{Hit, Hittable};
use crate::prelude::*;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn with(object: Arc<dyn Hittable>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let mut closest_hit = None;
        let mut closest_so_far = t_range.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, Interval::new(t_range.min, closest_so_far)) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
