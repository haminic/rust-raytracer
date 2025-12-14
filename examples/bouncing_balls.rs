use std::sync::Arc;

use rand::random_range;
use rust_raytracer::base::*;
use rust_raytracer::materials::*;
use rust_raytracer::objects::*;
use rust_raytracer::render::*;

static SAMPLES_PER_PIXEL: i32 = 150;
static MAX_DEPTH: i32 = 25;
static N_BALLS: i32 = 25;

fn main() -> std::io::Result<()> {
    let renderer = Renderer {
        max_samples: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        time_sampler: None,
    };

    let (world, camera) = bouncing_balls(N_BALLS, true);
    println!("Render Task #1: Multi-threaded, using BVH tree");
    let heatmap_file = get_output_file("bouncing_balls_mt_bvh")?;
    let file = get_output_file("bouncing_heatmap")?;
    renderer.multi_threaded_render(&camera, &world, file, Some(heatmap_file), None)?;

    // let (world, camera) = bouncing_balls(N_BALLS, false);
    // println!("Render Task #2: Multi-threaded, using array");
    // let file = get_output_file("bouncing_balls_mt")?;
    // renderer.multi_threaded_render(&camera, &world, file, None, None)?;

    // let (world, camera) = bouncing_balls(N_BALLS, false);
    // println!("Render Task #3: Single-threaded, using array (WARNING: Takes very long)");
    // let file = get_output_file("bouncing_balls_st")?;
    // renderer.single_threaded_render(&camera, &world, file, None, None)?;

    Ok(())
}

fn get_output_file(name: &str) -> std::io::Result<std::fs::File> {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("output");
    std::fs::create_dir_all(&path)?;
    path.push(format!("{name}.ppm"));
    std::fs::File::create(path)
}

pub fn bouncing_balls(n: i32, bvh: bool) -> (World, Camera) {
    let mut geometry = HittableList::new();

    let ground_material: Arc<dyn Material> = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    geometry.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -n..n {
        for b in -n..n {
            let choose_mat = random_range(0.0..1.0);
            let choose_bounce = random_range(0.0..1.0);
            let center = Point3::new(
                a as f64 + 0.9 * random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * random_range(0.0..1.0),
            );
            let translation = Vec3::new(0.0, random_range(0.0..0.5), 0.0);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_mat {
                    x if x < 0.5 => {
                        let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                        Lambertian::new(albedo)
                    }
                    x if x < 0.8 => {
                        let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                        Metal::new(albedo)
                    }
                    _ => Dielectric::new(1.5),
                };
                let sphere = Sphere::new(center, 0.2, sphere_material);
                if choose_bounce > 0.5 {
                    geometry.add(sphere);
                } else {
                    geometry.add(Translating::new(
                        sphere,
                        Vec3::new(0.0, 0.0, 0.0),
                        translation,
                    ));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    geometry.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    geometry.add(Sphere::new(Point3::new(2.0, 1.0, -2.5), 1.0, material2));

    let material3 = Metal::new(Metal::GOLD_ALBEDO);
    geometry.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material3));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(10.0, 2.0, 4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.6,
    );

    let backdrop_color = Color::new(0.70, 0.80, 1.00);

    let geometry = if bvh {
        to_hittable(Bvh::from_list(geometry))
    } else {
        to_hittable(geometry)
    };

    (World::new(backdrop_color, geometry), cam)
}
