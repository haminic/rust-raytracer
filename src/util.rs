use rand::random_range;
use rand_distr::{Distribution, Normal};

pub fn random_unit_f64() -> f64 {
    random_range(0.0..1.0)
}

pub fn random_normal_f64() -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    normal.sample(&mut rand::rng())
}

pub trait Randomable {
    const SUPREMUM: f64;
    fn eval(&self, _x: f64) -> f64;
    fn sample(&self) -> f64 {
        loop {
            let x: f64 = random_unit_f64();
            let choice: f64 = random_range(0.0..=Self::SUPREMUM);
            if choice <= self.eval(x) { return x; }
        }
    }
}

pub struct Uniform;
pub struct Custom;


impl Randomable for Uniform {
    const SUPREMUM: f64 = 1.0;
    fn eval(&self, _x: f64) -> f64 { 1.0 }
}

impl Randomable for Custom {
    const SUPREMUM: f64 = 2.0;
    fn eval(&self, x: f64) -> f64 { 3.0 * x * x }
}

pub struct Randomizer;
impl Randomizer {
    pub const UNIFORM: Uniform = Uniform;
    pub const CUSTOM: Custom = Custom;
}