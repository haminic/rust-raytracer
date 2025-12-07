use super::*;

pub fn cornell_box() -> (World, Camera) {
    let mut world = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));

    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let block1 = Block::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        white.clone(),
    );
    world.add(Rotated::new(
        block1,
        Point3::new(210.0, 0.0, 150.0),
        Axis::Y,
        20.0,
    ));

    world.add(Block::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        white.clone(),
    ));

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
