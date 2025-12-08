use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 20;
static MAX_DEPTH: i32 = 10;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("test_fog")?;

    let (world, camera) = test_fog();

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

pub fn test_fog() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    geometry.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let material1: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.2, 0.60, 0.2)));
    geometry.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material1));

    let difflight: Arc<dyn Material> =
        Arc::new(DiffuseLight::new(Color::new(0.73, 0.73, 0.73) * 2.0));
    geometry.add(Sphere::new(Point3::new(3.0, 1.0, -2.0), 1.5, difflight));

    let border: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let boundary: Sphere = Sphere::new(Point3::new(3.0, 1.5, 0.8), 1.8, border);
    geometry.add(ConstantMedium::new(
        boundary,
        1.2,
        Color::new(0.2, 0.4, 0.8),
    ));

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

    (
        World::new(Color::new(0.20, 0.20, 0.20) * 0.3, geometry),
        cam,
    )
}
