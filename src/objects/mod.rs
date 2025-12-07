mod base;
mod block;
mod bvh;
mod hittable_list;
mod quad;
mod rotate;
mod sphere;
mod translate;

pub use base::*;
pub use block::Block;
pub use bvh::Bvh;
pub use hittable_list::HittableList;
pub use quad::Quad;
pub use rotate::{Rotated, Rotating};
pub use sphere::Sphere;
pub use translate::{Translated, Translating};

use crate::prelude::*;

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit>;
    fn bounding_box(&self) -> Aabb;
}
