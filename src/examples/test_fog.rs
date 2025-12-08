use super::*;

pub fn test_fog() -> (World, Camera){
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material));
    
    let material1: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.60, 0.2)));
    world.add(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, material1));

    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(0.73, 0.73, 0.73) * 4.0 ));
    world.add(Sphere::new(Point3::new(3.0,1.0,-2.0), 1.5, difflight));

    let border: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let boundary: Sphere = Sphere::new(Point3::new(3.0, 1.5, 0.8), 1.8, border);
    world.add(ConstantMedium::new(boundary, 0.8, Color::new(0.2, 0.4, 0.8)));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(16.0, 3.0, 3.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.0,
    );

    (World::new(Color::new(0.20, 0.20, 0.20)*0.3, Box::new(world)), cam)
}