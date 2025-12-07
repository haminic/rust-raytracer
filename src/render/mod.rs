mod camera;
mod renderer;
mod world;

pub use camera::{Resolution, Camera};
pub use renderer::Renderer;
pub use world::World;

use crate::prelude::*;
use crate::objects::Hittable;

fn sample_square() -> Vec3 {
    Vec3::new(random_unit_f64() - 0.5, random_unit_f64() - 0.5, 0.0)
}

fn sample_in_unit_disk() -> Vec3 {
    let r = random_unit_f64().sqrt();
    let theta = random_range(0.0..(2.0 * PI));
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3::new(r * cos_theta, r * sin_theta, 0.0)
}
