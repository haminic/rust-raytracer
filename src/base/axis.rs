#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

    pub fn next(&self) -> Self {
        match self {
            Self::X => Self::Y,
            Self::Y => Self::Z,
            Self::Z => Self::X,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::X => Self::Z,
            Self::Y => Self::X,
            Self::Z => Self::Y,
        }
    }
}

impl rand_distr::Distribution<Axis> for rand_distr::StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        let axis = rng.random_range(0..3);
        match axis {
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
