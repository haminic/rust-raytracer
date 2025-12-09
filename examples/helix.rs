use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 5000;
static MAX_DEPTH: i32 = 10;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("helix")?;

    let (world, camera) = helix();

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

pub fn helix() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let red: Arc<dyn Material> = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white: Arc<dyn Material> = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green: Arc<dyn Material> = Lambertian::new(Color::new(0.12, 0.45, 0.15));

    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

    geometry.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green, // Right Wall
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red, // Left Wall
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(), // Floor
    ));
    geometry.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(), // Ceiling
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(), // Back Wall
    ));

    // Light Source
    geometry.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));

    // Helix Sphere
    let radius = 50.0;
    let room_center_x = 278.0;
    let room_center_z = 278.0;

    let box_center = Point3::new(room_center_x, radius, room_center_z);
    let sphere_pos = Point3::new(room_center_x, radius, room_center_z + 200.0);

    let mat: Arc<dyn Material> = Metal::new(Color::new(0.0, 0.0, 0.0));
    let helix_sphere = Translating::new(
        Rotating::new(
            Sphere::new(sphere_pos, radius, mat),
            box_center.clone(),
            Axis::Y,
            90.0,
            -90.0,
        ),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 200.0, 0.0),
    );

    geometry.add(helix_sphere);

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
