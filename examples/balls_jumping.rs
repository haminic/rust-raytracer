use std::sync::Arc;

use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 150;
static MAX_DEPTH: i32 = 30;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("balls_jumping")?;

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

static FOG: bool = true;
static ROTATE: bool = true;

pub fn test_fog() -> (World, Camera) {
    let mut geometry = HittableList::new();
    let mut moving = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.39, 0.26, 0.13)));

    geometry.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    let light_intensity: f64 = 40.0;
    let fuzz: f64 = 0.7;
    let glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5)); //try change to Lambertian
    let green: Arc<dyn Material> = Arc::new(Metal::with_fuzz(Color::new(0.1, 0.60, 0.1), fuzz));
    let red: Arc<dyn Material> = Arc::new(Metal::with_fuzz(Color::new(0.60, 0.1, 0.1), fuzz));
    let blue: Arc<dyn Material> = Arc::new(Metal::with_fuzz(Color::new(0.1, 0.1, 0.60), fuzz));
    let difflight: Arc<dyn Material> = Arc::new(DiffuseLight::new(Color::new(0.73, 0.73, 0.73) * light_intensity));

    let r: f64 = 2.0;
    let omega: f64 = 30.0;  // in degree
    let ball1 =  Sphere::new(Point3::new(0.0, 0.5, -1.0)* r, 0.5, green.clone());
    let ball2 =  Sphere::new(Point3::new(0.866, 0.5, 0.5) * r, 0.5, red.clone());
    let ball3 =  Sphere::new(Point3::new(-0.866, 0.5, 0.5) * r, 0.5, blue.clone());
    let ball_light = Sphere::new(Point3::new(0.0, 1.5, 0.0), 0.6, difflight.clone());
    let ball_glass =  Sphere::new(Point3::new(0.866, 1.0, -0.5) * 0.8 * r, 0.4, glass.clone());

    let boundary = Sphere::new(Point3::new(0.0, -0.6 * r, 0.0), 1.5 * r, red.clone());
    let fog = ConstantMedium::new(
        boundary,
        0.4, // density
        Color::new(0.2, 0.2, 0.2),
    );

    moving.add(ball1);
    moving.add(ball2);
    moving.add(ball3);
    moving.add(ball_glass);

    geometry.merge(moving);
    geometry.add(ball_light);

    if FOG { geometry.add(fog); }
    
    let resolution = Resolution::with_aspect_ratio(1.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(0.0, 20.0, 0.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        25.0,
        10.0,
        0.0,
    );

    (
        if !ROTATE { 
            World::new(Color::new(1.0, 1.0, 1.0 ) * 0.005, geometry) 
        } else { 
            let foo = Rotating::new(geometry, Point3::new(0.0,0.0,0.0), Axis::Y, 0.0, omega);
            World::new(Color::new(1.0, 1.0, 1.0 ) * 0.005, foo) 
        },
        cam,
    )
}
