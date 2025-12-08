use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 20;
static MAX_DEPTH: i32 = 10;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("pendulum")?;

    let (world, camera) = pendulum();

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

pub fn pendulum() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let red: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
    let green: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));
    
    geometry.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green, // Right Wall
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red,   // Left Wall
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone(), // Floor
    ));
    geometry.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone(), // Ceiling
    ));
    geometry.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone(), // Back Wall
    ));

    
    // Light Source
    geometry.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));

    // Pendulum
    let room_center_xz = 278.0;
    let ceiling_y = 555.0;
    let arm_length = 300.0;
    let sphere_radius = 50.0;
    
    let sphere_mat: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.9, 0.8, 0.2)));
    let arm_mat: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.3, 0.3, 0.3)));
    
    let pivot_center = Point3::new(room_center_xz, ceiling_y, 555.0);

    // Pendulum Arm
    let arm_thickness = 5.0;
    let arm_height = arm_length;

    // Arm coordinates (centered at X=278, Z=555, extends from ceiling down)
    let arm_p0 = Point3::new(
        room_center_xz - arm_thickness/2.0, 
        ceiling_y - arm_height, 
        pivot_center.z - arm_thickness/2.0
    );
    let arm_p1 = Point3::new(
        room_center_xz + arm_thickness/2.0, 
        ceiling_y, 
        pivot_center.z + arm_thickness/2.0
    );

    let arm_block = Block::new(arm_p0, arm_p1, arm_mat);

    // Pendulum Sphere
    let sphere_y_position = ceiling_y - arm_length - sphere_radius;
    let sphere_center = Point3::new(room_center_xz, sphere_y_position, pivot_center.z);

    let pendulum_sphere = Sphere::new(sphere_center, sphere_radius, sphere_mat);

    let mut pendulum = HittableList::new();
    pendulum.add(arm_block);
    pendulum.add(pendulum_sphere);

    // Rotating pendulum
    let start_angle = -15.0; 
    let end_angle = 15.0;

    let rotating_pendulum = Rotating::new(
        pendulum,
        pivot_center,
        Axis::Z, 
        start_angle,
        end_angle,
    );

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

    geometry.add(rotating_pendulum);

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)    
}
