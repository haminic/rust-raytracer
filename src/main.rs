mod color;
mod ray;
mod vec3;

use std::cmp;
use std::fs::File;
use std::io::{BufWriter, Write, stdout};

use crate::color::*;
use crate::ray::*;
use crate::vec3::*;

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Image setup
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = cmp::max((image_width as f64 / aspect_ratio) as i32, 1);

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
    let pixel100_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

    // Render
    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        let progress = j as f64 / (image_height - 1) as f64;
        show_progress(progress);
        for i in 0..image_width {
            let pixel_center =
                pixel100_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r);
            write_color(&mut writer, &pixel_color)?;
        }
    }

    println!("\nDone!");

    Ok(())
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
