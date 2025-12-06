mod base;
mod camera;
mod materials;
mod objects;
mod prelude;

use std::fs;

use crate::camera::{Camera, Renderer, Resolution};
use crate::materials::{Dielectric, Lambertian, Material};
use crate::objects::HittableList;
use crate::objects::Sphere;
use crate::prelude::*;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.50));
    let material_bubble: Rc<dyn Material> = Rc::new(Dielectric::new(1.00 / 1.50));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    )));

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 400);
    let cam = Camera::new(
        resolution,
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        3.4,
        10.0,
    );
    let renderer = Renderer::new(25, 25);
    let file = get_output_file()?;
    renderer.render(&cam, &world, file)?;

    // TODO: Show time elapsed.
    println!("Done.");

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

fn get_output_file() -> std::io::Result<File> {
    let mut path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
    path.push("output");
    fs::create_dir_all(&path)?;
    path.push("out.ppm");

    File::create(path)
}
