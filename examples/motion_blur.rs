use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 500;
static MAX_DEPTH: i32 = 50;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("motion_blur")?;

    let (world, camera) = motion_blur();

    renderer.multi_threaded_render(&camera, &world, file, None)?;

    Ok(())
}

fn get_output_file(name: &str) -> std::io::Result<std::fs::File> {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("output");
    std::fs::create_dir_all(&path)?;
    path.push(format!("{name}.ppm"));
    std::fs::File::create(path)
}

pub fn motion_blur() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let yellow = Arc::new(Lambertian::new(Color::new(0.9, 0.824, 0.114)));
    let blue = Arc::new(Lambertian::new(Color::new(0.118, 0.196, 0.922)));
    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));

    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    geometry.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        blue,
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        yellow,
    ));
    geometry.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    geometry.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let block1 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(120.0, 120.0, 120.0),
        white.clone(),
    );
    let block1 = Rotated::new(block1, Point3::new(0.0, 0.0, 0.0), Axis::Y, 15.0);
    let block1 = Rotated::new(block1, Point3::new(0.0, 0.0, 0.0), Axis::Z, 20.0);
    let block1 = Translating::new(
        block1,
        Vec3::new(265.0, 60.0, 150.0),
        Vec3::new(320.0, 100.0, 200.0),
    );
    geometry.add(block1);

    let block2 = Block::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(120.0, 120.0, 120.0),
        white.clone(),
    );
    let block2 = Rotated::new(block2, Point3::new(0.0, 0.0, 0.0), Axis::Y, -18.0);
    let block2 = Rotated::new(block2, Point3::new(0.0, 0.0, 0.0), Axis::Z, -30.0);
    let block2 = Translating::new(
        block2,
        Vec3::new(130.0, 350.0, 180.0),
        Vec3::new(100.0, 250.0, 190.0),
    );
    geometry.add(block2);

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

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)
}
