mod aabb;
mod hittable_list;
mod sphere;
mod hit;

pub use aabb::Aabb;
pub use hittable_list::HittableList;
pub use sphere::Sphere;
pub use hit::Hit;

use crate::{materials::Material, prelude::*};

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
