use super::*;

pub fn simple_light() -> (World, Camera){
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material));
    
    let material1: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, material1));

    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(4.0,4.0,4.0)));
    world.add(Sphere::new(Point3::new(3.0,1.0,-2.0), 1.5, difflight));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(26.0, 3.0, 6.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.0,
    );

    (World::new(Color::new(0.0, 0.0, 0.0), Box::new(world)), cam)
}