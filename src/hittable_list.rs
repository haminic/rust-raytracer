use std::rc::Rc;

use crate::hittable::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn with(object: Rc<dyn Hittable>) -> Self {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_range: std::ops::RangeInclusive<f64>,
        rec: &mut HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = *t_range.end();

        for object in &self.objects {
            if object.hit(ray, *t_range.start()..=closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
