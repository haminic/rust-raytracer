use std::sync::Arc;

use rand::random_range;
use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 1000;
static MAX_DEPTH: i32 = 50;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file("final_scene")?;

    let (world, camera) = final_scene();

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

pub fn final_scene() -> (World, Camera) {
    let mut boxes1 = HittableList::new();
    let ground_material: Arc<dyn Material> = Lambertian::new(Color::new(0.48, 0.83, 0.53));
    let boxes_per_side = 20;
    let w = 100.0;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;

            let y1 = random_range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Block::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground_material),
            ));
        }
    }

    let mut geometry = HittableList::new();

    geometry.add(Bvh::from_list(boxes1));

    let light_color = Color::new(7.0, 7.0, 7.0);
    let light_material = DiffuseLight::new(light_color);

    geometry.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light_material,
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Lambertian::new(Color::new(0.7, 0.3, 0.1));

    geometry.add(Translating::new(
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 50.0, sphere_material),
        center1,
        center2,
    ));

    geometry.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));

    geometry.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::with_fuzz(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary1 = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));

    geometry.add(boundary1.clone());
    geometry.add(ConstantMedium::new(
        boundary1,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));

    let boundary2 = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));

    geometry.add(Rotated::new(
        Rotated::new(
            Block::new(
                Point3::new(180.0, 220.0, 240.0),
                Point3::new(280.0, 340.0, 360.0),
                Metal::with_fuzz(Color::new(0.8, 0.4, 0.6), 0.2),
            ),
            Vec3::new(220.0, 280.0, 300.0),
            Axis::X,
            45.0,
        ),
        Vec3::new(220.0, 280.0, 300.0),
        Axis::Y,
        45.0,
    ));

    geometry.add(ConstantMedium::new(
        boundary2,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut boxes2 = HittableList::new();
    let white_material = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let ns = 1000;

    for _ in 0..ns {
        let random_point = Point3::new(
            random_range(0.0..165.0),
            random_range(0.0..165.0),
            random_range(0.0..165.0),
        );
        boxes2.add(Sphere::new(random_point, 10.0, white_material.clone()));
    }

    let bvh_of_spheres = Bvh::from_list(boxes2);
    let rotated_bvh = Rotated::new(
        bvh_of_spheres,
        Vec3::new(-100.0, 270.0, 395.0),
        Axis::Y,
        15.0,
    );
    let final_object = Translated::new(rotated_bvh, Vec3::new(-100.0, 270.0, 395.0));

    geometry.add(final_object);

    let resolution = Resolution::with_aspect_ratio(1.0, 800);
    let cam = Camera::new(
        resolution,
        Point3::new(478.0, 278.0, -600.0),
        Point3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        10.0,
        0.0,
    );

    (World::new(Color::new(0.0, 0.0, 0.0), geometry), cam)
}
