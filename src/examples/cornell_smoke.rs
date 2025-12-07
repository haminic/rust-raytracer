use super::*;

pub fn cornell_smoke() -> (World, Camera) {
    let mut world = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    world.add(Quad::new(Point3::new(555.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), Vec3::new(0.0,0.0,555.0), green));
    world.add(Quad::new(Point3::new(0.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), Vec3::new(0.0,0.0,555.0), red));
    world.add(Quad::new(Point3::new(113.0, 554.0, 127.0), Vec3::new(330.0,0.0,0.0), Vec3::new(0.0,0.0,305.0), light));
    world.add(Quad::new(Point3::new(0.0,555.0,0.0), Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,0.0,555.0), white.clone()));
    world.add(Quad::new(Point3::new(0.0,0.0,0.0), Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,0.0,555.0), white.clone()));
    world.add(Quad::new(Point3::new(0.0,0.0,555.0), Vec3::new(555.0,0.0,0.0), Vec3::new(0.0,555.0,0.0), white.clone()));

    let box1 = Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.00, 330.0, 165.0), white.clone());
    let box1 = Rotated::new(box1, Point3::new(0.0, 0.0, 0.0), Axis::Y, 15.0);
    let box1 = Translated::new(box1, Vec3::new(256.0, 0.0, 295.0));
    
    let box2 = Block::new(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Rotated::new(box2, Point3::new(0.0, 0.0, 0.0), Axis::Y, -18.0);
    let box2 = Translated::new(box2, Vec3::new(130.0, 0.0, 65.0));
    
    world.add(ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0)));
    world.add(ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0)));

    let resolution = Resolution::with_aspect_ratio(1.0, 600);
    let cam = Camera::new(
        resolution,
        Point3::new(278.0, 278.0, -800.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        10.0,
        0.0,
    );

    (World::new(Color::new(0.0, 0.0, 0.0), Box::new(world)), cam)
}
