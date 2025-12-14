use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static MAX_DEPTH: u32 = 5;

fn main() -> std::io::Result<()> {
    let renderer = Renderer {
        samples_range: (8, 1000),
        max_depth: MAX_DEPTH,
        time_sampler: Some(halton_sampler(2)),
        tolerable_cv: 0.01,
    };
    let file = get_output_file("transform_test")?;

    let (world, camera) = traffic_light();

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

pub fn traffic_light() -> (World, Camera) {
    let mut geometry = HittableList::new();

    let white_material = Lambertian::new(Color::new(0.9, 0.9, 0.9));
    geometry.add(Quad::new(
        Point3::new(-1000.0, 0.0, -1000.0),
        Vec3::new(2000.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2000.0),
        white_material.clone(),
    ));
    // middle wall
    geometry.add(Block::new(
        Point3::new(-1000.0, 0.0, 0.0),
        Point3::new(-2.0, 5.0, -1.0),
        white_material.clone(),
    ));
    geometry.add(Block::new(
        Point3::new(1000.0, 0.0, 0.0),
        Point3::new(2.0, 5.0, -1.0),
        white_material.clone(),
    ));
    // back wall
    geometry.add(Block::new(
        Point3::new(-1000.0, 0.0, -8.0),
        Point3::new(1000.0, 8.0, -9.0),
        white_material.clone(),
    ));

    let light_intensity: f64 = 100.0;
    let others = 0.01;
    let red_light = DiffuseLight::new(Color::new(1.0, others, others) * light_intensity);
    let green_light = DiffuseLight::new(Color::new(others, 1.0, others) * light_intensity);
    let blue_light = DiffuseLight::new(Color::new(others, others, 1.0) * light_intensity);

    let red_ball = Sphere::new(Point3::new(-5.0, 1.0, 7.0), 1.0, red_light.clone());
    let green_ball = Sphere::new(Point3::new(0.0, 1.0, 8.0), 1.0, green_light.clone());
    let blue_ball = Sphere::new(Point3::new(5.0, 1.0, 7.0), 1.0, blue_light.clone());

    geometry.add(red_ball);
    geometry.add(green_ball);
    geometry.add(blue_ball);

    // Light source
    let white_light = DiffuseLight::new(Color::new(1.0, 1.0, 1.0) * 200.0);
    let white_ball = Sphere::new(Point3::new(0.0, 15.0, 0.0), 1.0, white_light);
    geometry.add(white_ball);

    let sphere = Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        Metal::new(Vec3::new(1.0, 1.0, 1.0)),
    );
    // let sphere = Scaling::new(sphere, Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 1.0, 1.0), Vec3::new(1.0, 3.0, 1.0));
    let transform1 = Mat3::new([[3.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);

    let transform2 = Mat3::new([[1.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 1.0]]);
    let sphere = Transforming::new(sphere, transform1, transform2).unwrap();
    let sphere = Translated::new(sphere, Vec3::new(0.0, 5.0, 0.0));

    geometry.add(sphere);

    let resolution = Resolution::with_aspect_ratio(1.0, 1200);
    let position = CameraPosition {
        look_from: Point3::new(0.0, 15.0, 10.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        up_direction: Vec3::new(0.0, 1.0, 0.0),
    };
    let settings = CameraSettings::with_fov(80.0);
    let cam = Camera::new(position, resolution, settings);

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)
}
