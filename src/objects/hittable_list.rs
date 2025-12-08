use crate::objects::{Aabb, Hit, Hittable};
use crate::prelude::*;

/*
    Contain all objects in the world
    Checking by iterate over everything : O(n)
*/

pub struct HittableList {
    bbox: Aabb,
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: Aabb::EMPTY,
        }
    }

    pub fn with(object: impl Hittable + 'static) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = Aabb::enclosing(self.bbox, object.bounding_box());
        self.objects.push(Box::new(object));
    }

    pub fn merge(&mut self, mut other: HittableList) {
        self.bbox = Aabb::enclosing(self.bbox, other.bounding_box());
        self.objects.append(&mut other.objects);
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

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
