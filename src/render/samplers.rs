use crate::prelude::*;

// --- LOW DISCREPENCY SEQUENCES ---
pub type SampleFn = Arc<dyn Fn(u32) -> f64 + Send + Sync>;

pub fn uniform_sampler() -> SampleFn {
    Arc::new(move |_| random_unit_f64())
}

pub fn halton(mut index: u32, base: u32) -> f64 {
    let mut f = 1.0;
    let mut result = 0.0;

    while index > 0 {
        f /= base as f64;
        result += f * (index % base) as f64;
        index /= base;
    }

    result
}

pub fn halton_sampler(base: u32) -> SampleFn {
    Arc::new(move |idx| halton(idx, base))
}

pub fn sobol(n: u32) -> f64 {
    let mut result = 0.0;
    let mut f = 0.5;
    let mut i = n;

    while i != 0 {
        if i & 1 != 0 {
            result += f;
        }
        i >>= 1;
        f *= 0.5;
    }

    result
}

pub fn sobol_sampler() -> SampleFn {
    Arc::new(move |idx| sobol(idx))
}

/// Returns a sampler that draws samples according to the given PDF in [0, max)
pub fn sampler_from_randomable(rv: impl Randomable + 'static) -> SampleFn {
    Arc::new(move |_| rv.sample())
}

pub trait Randomable: Send + Sync {
    const SUPREMUM: f64;
    fn eval(&self, _x: f64) -> f64;
    fn sample(&self) -> f64 {
        loop {
            let x: f64 = random_unit_f64();
            let choice: f64 = rand::random_range(0.0..=Self::SUPREMUM);
            if choice <= self.eval(x) {
                return x;
            }
        }
    }
}

/*
    Random Function should be a Probability Function (Integral 0 -> 1 equals 1)
    The expected call time for sample() is Supremum of f(x); x = 0..1
*/

pub struct Uniform;
pub struct Custom;
pub struct Logistic;

impl Randomable for Uniform {
    const SUPREMUM: f64 = 1.0;
    fn eval(&self, _x: f64) -> f64 {
        1.0
    }
}

impl Randomable for Custom {
    const SUPREMUM: f64 = 2.0;
    fn eval(&self, x: f64) -> f64 {
        3.0 * x * x
    }
}

impl Randomable for Logistic {
    const SUPREMUM: f64 = 2.0;
    fn eval(&self, x: f64) -> f64 {
        2.0 / (1.0 + (-6.0 * x + 3.0).exp())
    }
}

pub struct Randomizer;
impl Randomizer {
    pub const UNIFORM: Uniform = Uniform;
    pub const CUSTOM: Custom = Custom;
    pub const LOGISTIC: Logistic = Logistic;
}
