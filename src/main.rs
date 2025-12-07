mod base;
mod examples;
mod materials;
mod objects;
mod prelude;
mod render;

use std::fs;
use std::time::Instant;

use crate::examples::*;
use crate::prelude::*;
use crate::render::*;

static SAMPLES_PER_PIXEL: i32 = 200; // For cornell_box should be 200
static MAX_DEPTH: i32 = 10; // For cornell_box should be 50

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file()?;

<<<<<<< HEAD
    let (world, camera) = bouncing_balls(10);
    // let (world, camera) = simple_light();
    // let (world, camera) = cornell_box();
=======
    // let (world, camera) = bouncing_balls(10);
    // let (world, camera) = simple_light();
    let (world, camera) = cornell_box();
>>>>>>> 4db185e (Implement Rotated and Rotating)

    let start_time = Instant::now();
    renderer.render(&camera, &world, file)?;

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
