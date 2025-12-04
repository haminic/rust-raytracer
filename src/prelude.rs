pub use std::fs::File;
pub use std::io::{BufWriter, Write, stdout};
pub use std::rc::Rc;

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

pub use rand::random_range;

pub use crate::base::color::*;
pub use crate::base::interval::*;
pub use crate::base::ray::*;
pub use crate::base::vec3::*;

pub fn random_f64() -> f64 {
    random_range(0.0..1.0)
}

pub fn random_normal_f64() -> f64 {
    use rand_distr::{Distribution, Normal};
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.sample(&mut rand::rng())
}
