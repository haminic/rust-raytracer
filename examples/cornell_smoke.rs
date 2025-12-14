use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static MAX_DEPTH: u32 = 50;

fn main() -> std::io::Result<()> {
    let renderer = Renderer {
        samples_range: (8, 150),
        max_depth: MAX_DEPTH,
        time_sampler: Some(halton_sampler(2)),
        tolerable_cv: 0.01,
    };
    let file = get_output_file("cornell_smoke")?;

    let (world, camera) = cornell_smoke();

    renderer.multi_threaded_render(&camera, &world, file, None, None)?;

    Ok(())
}

fn get_output_file(name: &str) -> std::io::Result<std::fs::File> {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("output");
    std::fs::create_dir_all(&path)?;
    path.push(format!("{name}.ppm"));
    std::fs::File::create(path)
}

pub fn cornell_smoke() -> (World, Camera) {
    let mut world = HittableList::new();

    let red: Arc<dyn Material> = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white: Arc<dyn Material> = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green: Arc<dyn Material> = Lambertian::new(Color::new(0.12, 0.45, 0.15));

    let light = DiffuseLight::new(Color::new(7.0, 7.0, 7.0));

    // walls
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
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = HittableBuilder::new(Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.00, 330.0, 165.0),
        white.clone(),
    ))
    .rotate_y(15.0)
    .translate(Vec3::new(256.0, 0.0, 295.0))
    .build();
    world.add(ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0)));

    let box2 = HittableBuilder::new(Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ))
    .rotate_y(-18.0)
    .translate(Vec3::new(130.0, 0.0, 65.0))
    .build();
    world.add(ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0)));

    let resolution = Resolution::square(600);
    let position = CameraPosition {
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings::with_fov(40.0);
    let cam = Camera::new(position, resolution, settings);

    (World::new(Color::new(0.0, 0.0, 0.0), world), cam)
}
