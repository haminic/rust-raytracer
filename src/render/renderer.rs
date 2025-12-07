use super::*;

use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Renderer {
    pub(super) samples_per_pixel: i32,
    pub(super) pixel_samples_scale: f64,
    pub(super) max_depth: i32,
}

impl Renderer {
    pub fn new(samples_per_pixel: i32, max_depth: i32) -> Self {
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        Self {
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, camera: &Camera, world: &World, file: File) -> std::io::Result<()> {
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        let pb = Arc::new(ProgressBar::new(
            camera.resolution.width as u64 * camera.resolution.height as u64,
        ));
        pb.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{bar:40}] {percent:>3}%")
                .unwrap()
                .progress_chars("#>-"),
        );

        let pixel_colors: Vec<Color> = (0..camera.resolution.height)
            .into_par_iter()
            .flat_map(|j| {
                let pb = pb.clone();
                (0..camera.resolution.width).into_par_iter().map(move |i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let ray = camera.sample_ray(i, j);
                        pixel_color += ray_color(&ray, self.max_depth, world)
                    }

                    // Return the pixel color
                    pb.inc(1);
                    self.pixel_samples_scale * pixel_color
                })
            })
            .collect();

        for pixel_color in pixel_colors {
            write_color(&mut writer, pixel_color)?;
        }

        pb.finish();
        println!("\nDone!");
        Ok(())
    }
}

fn ray_color(ray: &Ray, depth: i32, world: &World) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    //TODO: the return logic is very suspicious and I dont know how to fix it yet. I will come back later
    if let Some(hit) = world.geometry.hit(ray, Interval::new(0.001, INFINITY)) {
        // hit something
        let emission = hit.mat.emitted(hit.point);
        if let Some(scatter) = hit.mat.scatter(ray, &hit) {
            emission + scatter.attenuation * ray_color(&scatter.ray_out, depth - 1, world)
        } else {
            emission
        }
    } else {
        world.backdrop
    }
}