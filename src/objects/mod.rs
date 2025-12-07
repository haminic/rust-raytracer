mod base;
mod bvh;
mod hittable_list;
mod quad;
mod rotated;
mod sphere;
mod translated;
mod block;

pub use base::*;
pub use bvh::Bvh;
pub use hittable_list::HittableList;
pub use quad::Quad;
pub use rotated::Rotated;
pub use sphere::Sphere;
pub use translated::Translated;
pub use block::Block;

use crate::prelude::*;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
