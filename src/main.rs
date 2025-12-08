// TODO: remove later
#![allow(unused_imports)]
#![allow(dead_code)]

mod base;
mod examples;
mod materials;
mod objects;
mod prelude;
mod render;

use std::fs;
use std::time::Instant;
use std::io::{ErrorKind::InvalidInput, Error};

use crate::examples::*;
use crate::prelude::*;
use crate::render::*;

static SAMPLES_PER_PIXEL: i32 = 20; // For cornell_box should be 200
static MAX_DEPTH: i32 = 10; // For cornell_box should be 50
static SCENE_SELECTED: i32 = 3;

fn main() -> std::io::Result<()> {
    let renderer = Renderer::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let file = get_output_file()?;

    let (world, camera) = match SCENE_SELECTED {
        1 => bouncing_balls(10),
        2 => simple_light(),
        3 => test_fog(),
        4 => cornell_box(),
        5 => cornell_smoke(),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "The selected scene does not exist",
            ));
        }
    };

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
