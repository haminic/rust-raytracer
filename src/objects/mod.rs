mod base;
mod block;
mod bvh;
mod hittable_list;
mod quad;
mod sphere;
mod translated;
mod translating;
mod constant_medium;

pub use base::*;
pub use block::Block;
pub use bvh::Bvh;
pub use hittable_list::HittableList;
pub use quad::Quad;
pub use sphere::Sphere;
pub use translated::Translated;
pub use translating::Translating;
pub use constant_medium::ConstantMedium;

use crate::prelude::*;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
