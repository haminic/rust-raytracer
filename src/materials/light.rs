use crate::prelude::*;
use super::Material;

pub struct DiffuseLight {
    color: Color,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        DiffuseLight { color: emit }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, _: Point3) -> Color {
        self.color
    }
}
