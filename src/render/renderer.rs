use super::*;

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

pub struct Renderer {
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
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

    pub fn multi_threaded_render(
        &self,
        camera: &Camera,
        world: &World,
        file: File,
        style: ProgressStyle,
    ) -> std::io::Result<()> {
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        let total_pixels = camera.resolution.width * camera.resolution.height;
        let pb = Arc::new(ProgressBar::new(total_pixels as u64));
        pb.set_style(style);

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
                    pb.inc(1);

                    // Return the pixel color
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

    pub fn single_threaded_render(
        &self,
        camera: &Camera,
        world: &World,
        file: File,
        style: ProgressStyle,
    ) -> std::io::Result<()> {
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        let total_pixels = camera.resolution.width * camera.resolution.height;
        let pb = ProgressBar::new(total_pixels as u64);
        pb.set_style(style);

        for j in 0..camera.resolution.height {
            for i in 0..camera.resolution.width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = camera.sample_ray(i, j);
                    pixel_color += ray_color(&ray, self.max_depth, world);
                }

                let scaled_color = self.pixel_samples_scale * pixel_color;
                write_color(&mut writer, scaled_color)?;
                pb.inc(1);
            }
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
