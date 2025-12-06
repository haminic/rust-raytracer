mod base;
mod camera;
mod materials;
mod objects;
mod prelude;

use std::fs;
use std::time::Instant;

use crate::camera::{Camera, Renderer, Resolution};
use crate::materials::{Dielectric, Lambertian, Material};
use crate::objects::HittableList;
use crate::objects::Sphere;
use crate::prelude::*;

static SAMPLES_PER_PIXEL: i32 = 10;
static MAX_DEPTH: i32 = 10;

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();

    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let choose_bounce = random_f64();
            let center1 = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            let center2 = center1 + Vec3::new(0.0, random_range(0.0..0.5), 0.0);

            if (center1 - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_mat {
                    x if x < 0.8 => {
                        let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                        Arc::new(Lambertian::new(albedo))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };
                if choose_bounce > 0.5 {
                    world.add(Arc::new(Sphere::new(center1, 0.2, sphere_material)));
                } else {
                    world.add(Arc::new(Sphere::new_moving(
                        center1,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(13.0, 2.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.6,
    );
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file()?;
    renderer.render(&cam, &world, file)?;

    let elapsed = start_time.elapsed().as_millis();
    println!("Render time = {}.{} s", elapsed / 1000, elapsed % 1000);

    Ok(())
}

fn get_output_file() -> std::io::Result<File> {
    let mut path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("output");
    fs::create_dir_all(&path)?;
    path.push("out.ppm");

    File::create(path)
}
