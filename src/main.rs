mod base;
mod camera;
mod materials;
mod objects;
mod prelude;

use std::fs;

use crate::camera::Camera;
use crate::materials::{Lambertian, Material};
use crate::objects::hittable_list::HittableList;
use crate::objects::sphere::Sphere;
use crate::prelude::*;

fn main() -> std::io::Result<()> {
    let mut world = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));

    let cam = Camera::new(16.0 / 9.0, 400, 100, 50);
    let file = get_output_file()?;
    let mut writer = BufWriter::new(file);
    cam.render(&mut writer, &world)?;

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
