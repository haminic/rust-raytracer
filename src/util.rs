use rand::random_range;
use rand_distr::{Distribution, Normal};

pub fn random_unit_f64() -> f64 {
    random_range(0.0..1.0)
}

pub fn random_normal_f64() -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.sample(&mut rand::rng())
}
