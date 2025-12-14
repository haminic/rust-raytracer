use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static MAX_DEPTH: u32 = 10;

fn main() -> std::io::Result<()> {
    let renderer = Renderer {
        samples_range: (8, 1000),
        max_depth: MAX_DEPTH,
        time_sampler: Some(halton_sampler(2)),
        tolerable_cv: 0.01,
    };
    let file = get_output_file("simple_light")?;

    let (world, camera) = simple_light();

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

pub fn simple_light() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let ground_material: Arc<dyn Material> = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    geometry.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let material1: Arc<dyn Material> = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    geometry.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material1));

    let difflight: Arc<dyn Material> = DiffuseLight::new(Color::new(4.0, 4.0, 4.0));
    geometry.add(Sphere::new(Point3::new(3.0, 1.0, -2.0), 1.5, difflight));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let position = CameraPosition {
        look_from: Point3::new(26.0, 3.0, 6.0),
        look_at: Point3::new(0.0, 2.0, 0.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings::with_fov(20.0);
    let cam = Camera::new(position, resolution, settings);

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)
}
