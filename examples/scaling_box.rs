use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: u32 = 100;
static MAX_DEPTH: u32 = 5;

fn main() -> std::io::Result<()> {
    let renderer = Renderer {
        max_samples: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        time_sampler: Some(halton_sampler(2)),
    };
    let file = get_output_file("scaling_box")?;

    let (world, camera) = cornell_box();

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

pub fn cornell_box() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let red: Arc<dyn Material> = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white: Arc<dyn Material> = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green: Arc<dyn Material> = Lambertian::new(Color::new(0.12, 0.45, 0.15));

    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

    geometry.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
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

    let mat = Metal::with_fuzz(Vec3::new(0.8, 0.6, 0.7), 1.0);
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 50.0, mat);
    let transform = Mat3::new([[2.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let sphere = Transformed::new(sphere, transform).unwrap();
    let sphere = Translated::new(sphere, Vec3::new(278.0, 278.0, 400.0));

    geometry.add(sphere);

    let resolution = Resolution::square(600);
    let position = CameraPosition {
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings::with_fov(40.0);
    let cam = Camera::new(position, resolution, settings);

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)
}
