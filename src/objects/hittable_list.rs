use crate::objects::prelude::*;

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
        ray: &crate::base::ray::Ray,
        t_range: Interval,
        rec: &mut crate::objects::hittable::HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_range.max;

        for object in &self.objects {
            if object.hit(ray, Interval::from(0.0, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
