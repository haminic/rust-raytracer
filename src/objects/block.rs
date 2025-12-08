use crate::materials::Material;
use crate::objects::{Aabb, Hit, Hittable, HittableList, Quad};
use crate::prelude::*;

pub struct Block {
    faces: HittableList,
}

impl Block {
    pub fn new(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Self {
        let mut faces = HittableList::new();

        let min = Point3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));

        let max = Point3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        // Front
        faces.add(Quad::new(
            Point3::new(min.x, min.y, max.z),
            dx,
            dy,
            mat.clone(),
        ));

        // Right
        faces.add(Quad::new(
            Point3::new(max.x, min.y, max.z),
            -dz,
            dy,
            mat.clone(),
        ));

        // Back
        faces.add(Quad::new(
            Point3::new(max.x, min.y, min.z),
            -dx,
            dy,
            mat.clone(),
        ));

        // Left
        faces.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dz,
            dy,
            mat.clone(),
        ));

        // Top
        faces.add(Quad::new(
            Point3::new(min.x, max.y, max.z),
            dx,
            -dz,
            mat.clone(),
        ));

        // Bottom
        faces.add(Quad::new(
            Point3::new(min.x, min.y, min.z),
            dx,
            dz,
            mat.clone(),
        ));

        Self { faces }
    }
}

impl Hittable for Block {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        self.faces.hit(ray, t_range)
    }

    fn bounding_box(&self) -> Aabb {
        self.faces.bounding_box()
    }
}
