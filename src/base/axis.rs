use rand_distr::{Distribution, StandardUniform};

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
}

impl Distribution<Axis> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        let axis = rng.random_range(0..3);
        match axis {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
