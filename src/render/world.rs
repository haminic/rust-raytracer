use super::*;

pub struct World {
    pub backdrop: Color,
    pub geometry: Box<dyn Hittable>,
}

impl World {
    pub fn new(backdrop: Color, geometry: Box<dyn Hittable>) -> Self {
        Self { backdrop, geometry }
    }
}