use super::*;

pub struct World {
    pub backdrop: Color,
    pub geometry: Box<dyn Hittable>,
}

impl World {
    pub fn new(backdrop: Color, geometry: impl Hittable + 'static) -> Self {
        Self {
            backdrop,
            geometry: Box::new(geometry),
        }
    }
}
