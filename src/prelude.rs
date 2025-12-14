pub use std::sync::Arc;

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

pub use crate::base::*;
pub use crate::util::*;

pub fn random_unit_f64() -> f64 {
    rand::random_range(0.0..1.0)
}
