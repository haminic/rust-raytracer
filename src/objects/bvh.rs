use std::cmp::Ordering;

use super::{Aabb, Hit, Hittable};
use crate::{objects::HittableList, prelude::*};

pub enum Bvh {
    Node {
        left: Box<Bvh>,
        right: Box<Bvh>,
        bbox: Aabb,
    },
    Leaf(Box<dyn Hittable>),
}

impl Bvh {
    pub fn from_list(hittable_list: HittableList) -> Self {
        Self::new(hittable_list.objects)
    }

    pub fn new(mut objects: Vec<Box<dyn Hittable>>) -> Self {
        let axis = rand::random();
        let size = objects.len();

        match size {
            1 => Self::Leaf(objects.pop().unwrap()),
            2 => {
                let bbox = Aabb::enclosing(objects[0].bounding_box(), objects[1].bounding_box());
                Self::Node {
                    left: Box::new(Self::Leaf(objects.pop().unwrap())),
                    right: Box::new(Self::Leaf(objects.pop().unwrap())),
                    bbox,
                }
            }
            _ => {
                objects.sort_by(|a, b| box_compare(a, b, axis));
                let mid = size / 2;
                let right_vec = objects.split_off(mid);
                let left = Box::new(Self::new(objects));
                let right = Box::new(Self::new(right_vec));
                let bbox = Aabb::enclosing(left.bounding_box(), right.bounding_box());
                Self::Node { left, right, bbox }
            }
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        if !self.bounding_box().hit(ray, t_range) {
            return None;
        }

        match self {
            Self::Node { left, right, .. } => {
                let hit_left = left.hit(&ray, t_range);
                let hit_right = right.hit(&ray, t_range);
                match (hit_left, hit_right) {
                    (Some(l), Some(r)) => {
                        if l.t < r.t {
                            Some(l)
                        } else {
                            Some(r)
                        }
                    }
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }
            Self::Leaf(leaf) => leaf.hit(&ray, t_range),
        }
    }

    fn bounding_box(&self) -> Aabb {
        match self {
            Self::Node { bbox, .. } => *bbox,
            Self::Leaf(leaf) => leaf.bounding_box(),
        }
    }
}

fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: Axis) -> Ordering {
    let a_axis_interval = a.bounding_box().axis(axis);
    let b_axis_interval = b.bounding_box().axis(axis);
    a_axis_interval
        .min
        .partial_cmp(&b_axis_interval.min)
        .unwrap()
}
