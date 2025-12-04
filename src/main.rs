mod base;
mod camera;
mod objects;
mod prelude;

use std::fs;

use crate::camera::Camera;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::objects::hittable_list::HittableList;
use crate::objects::sphere::Sphere;
use crate::prelude::*;

fn main() -> std::io::Result<()> {
    // Image setup
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
    let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

    // Render
    let file = get_output_file()?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        let progress = j as f64 / (image_height - 1) as f64;
        show_progress(progress);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            write_color(&mut writer, &pixel_color)?;
        }
    }
    println!("\nDone!");

    Ok(())
}

fn new_main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400);
    let file = get_output_file()?;
    let mut writer = BufWriter::new(file);
    cam.render(&mut writer, &world);

    Ok(())
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(ray, Interval::from(0.0, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn show_progress(progress: f64) {
    let bar_width = 25;
    let filled = (progress * bar_width as f64) as usize;

    let bar = format!(
        "[{}{}] {:3}%",
        "=".repeat(filled),
        " ".repeat(bar_width - filled),
        (progress * 100.0) as i32
    );

    print!("\r{}", bar);
    stdout().flush().unwrap();
}

fn get_output_file() -> std::io::Result<File> {
    let mut path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("output");
    fs::create_dir_all(&path)?;
    path.push("out.ppm");

    File::create(path)
}
