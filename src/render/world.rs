use super::*;

pub struct World {
    pub(super) backdrop: Color,
    pub(super) geometry: Box<dyn Hittable>,
}

impl World {
    pub fn new(backdrop: Color, geometry: Box<dyn Hittable>) -> Self {
        Self { backdrop, geometry }
    }
}