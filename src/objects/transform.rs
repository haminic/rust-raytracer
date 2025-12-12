use crate::objects::{Hit, Hittable, Aabb};
use crate::prelude::*;

pub struct  Transformed<T> {
    object: T,
    transform: Mat3,
    inv: Mat3,
    bbox: Aabb,
}

impl<T: Hittable> Transformed<T> {
    pub fn new(object: T, transform: Mat3) -> Option<Self> {
        let bbox = &transform * object.bounding_box();
        let inv = transform.inverse();

        inv.map(|inv| {
            Self {
                object,
                transform,
                inv,
                bbox
            }
        })
    }
}

impl<T: Hittable> Hittable for Transformed<T> {
    fn hit(&self, ray: &crate::prelude::Ray, t_range: crate::prelude::Interval) -> Option<Hit> {
        let transformed_ray = Ray::with_time(&self.inv * ray.origin, &self.inv * ray.direction, ray.time);
        
        self.object.hit(&transformed_ray, t_range).map(|mut hit| {
            hit.point = &self.transform * hit.point;
            hit.normal = (&self.inv.transpose() * hit.normal).unit_vector();
            hit
        })
    }

    fn bounding_box(&self) -> super::Aabb {
        self.bbox
    }
}

pub struct Transforming<T> {
    object: T,
    transform: Lerp<Mat3>,
    inv: Lerp<Mat3>,
    bbox: Aabb,
}

impl<T: Hittable> Transforming<T> {
    pub fn new(object: T, transform1: Mat3, transform2: Mat3) -> Option<Self> {
        let bbox1 = &transform1 * object.bounding_box();
        let bbox2 = &transform2 * object.bounding_box();

        let inv1 = transform1.inverse();
        let inv2 = transform2.inverse();

        if !(inv1.is_none() || inv2.is_none()) {
            Some(Self {
                object,
                transform: Lerp::new(transform1, transform2),
                inv: Lerp::new(inv1.unwrap(), inv2.unwrap()),
                bbox: Aabb::enclosing(bbox1, bbox2)
            })
        }
        else {
            None
        }
        
    }
}

impl<T: Hittable> Hittable for Transforming<T>{
    fn hit(&self, ray: &crate::prelude::Ray, t_range: crate::prelude::Interval) -> Option<Hit> {
        let transform = self.transform.at(ray.time);
        let inv = transform.inverse().unwrap();
        let transformed_ray = Ray::with_time(&inv * ray.origin, &inv * ray.direction, ray.time);
        self.object.hit(&transformed_ray, t_range).map(|mut hit| {
            hit.point = &transform * hit.point;
            hit.normal = (&inv.transpose() * hit.normal).unit_vector();
            hit
        })
    }

    fn bounding_box(&self) -> super::Aabb {
        self.bbox        
    }
}
        