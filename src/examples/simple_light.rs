use crate::camera::*;
use crate::materials::*;
use crate::objects::*;
use crate::prelude::*;

pub fn simple_light() -> (World, Camera){
    let mut world = HittableList::new();

    let pertext: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, pertext));
    // world.add(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, pertext));

    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(4.0,4.0,4.0)));
    // world.add(Quad::new(Point3::new(3.0,1.0,-2.0), Vec3::new(2.0,0.0,0.0), Vec3::new(0.0,2.0,0.0), difflight));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 400);
    let cam = Camera::new(
        resolution,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.0,
    );

    (World::new(Color::new(0.70, 0.80, 1.00), Box::new(world)), cam)
}