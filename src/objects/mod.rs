mod aabb;
mod bvh;
mod hit;
mod hittable_list;
mod sphere;

pub use aabb::Aabb;
pub use bvh::Bvh;
pub use hit::Hit;
pub use hittable_list::HittableList;
pub use sphere::Sphere;

use crate::{materials::Material, prelude::*};

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
