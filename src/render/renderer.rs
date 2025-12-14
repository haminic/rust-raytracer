pub use std::fs::File;
pub use std::io::{BufWriter, Write};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::prelude::*;
use crate::render::Camera;
use crate::render::World;

pub struct Renderer {
    pub max_samples: i32,
    pub max_depth: i32,
    pub time_sampler: Option<SampleFn>,
}

impl Renderer {
    const MIN_SAMPLES: i32 = 8; // > 0
    const TOLERABLE_CV: f64 = 0.01; // Tolerable coefficiant of variation

    pub fn multi_threaded_render(
        &self,
        camera: &Camera,
        world: &World,
        file: File,
        heatmap_file: Option<File>,
        style: Option<ProgressStyle>,
    ) -> std::io::Result<()> {
        // --- PROGRESS BAR ---
        let style = style.unwrap_or(
            ProgressStyle::with_template("[{elapsed_precise}] [{bar:40}] {percent:>3}%")
                .unwrap()
                .progress_chars("#>-"),
        );

        let total_pixels = camera.resolution.width * camera.resolution.height;
        let pb = Arc::new(ProgressBar::new(total_pixels as u64));
        pb.set_style(style);

        // --- MAIN LOOP ---
        let pixel_colors: Vec<(Color, i32)> = (0..camera.resolution.height)
            .into_par_iter()
            .flat_map(|j| {
                let pb = pb.clone();
                (0..camera.resolution.width).into_par_iter().map(move |i| {
                    let mut stats = RunningStats::new();

                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for s in 0..self.max_samples {
                        let time = match &self.time_sampler {
                            None => random_unit_f64(),
                            Some(sampler) => sampler(s + i + j * camera.resolution.width),
                        };
                        let ray = camera.sample_ray(i, j, time);
                        let ray_color = ray_color(&ray, self.max_depth, world);
                        pixel_color += ray_color;
                        let luminance =
                            0.2126 * ray_color.x + 0.7152 * ray_color.y + 0.0722 * ray_color.z;
                        // Update stats
                        stats.add(luminance);

                        if stats.n >= Self::MIN_SAMPLES {
                            // standard error
                            let se = (stats.variance() / stats.n as f64).sqrt();
                            if se < Self::TOLERABLE_CV * stats.mean.max(1e-3) {
                                break;
                            }
                        }
                    }
                    pb.inc(1);

                    // Return the pixel color and number of samples
                    (pixel_color / stats.n as f64, stats.n)
                })
            })
            .collect();

        // --- WRITE TO MAIN FILE ---
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        let mut max_samples_used = 0;
        for &(color, samples) in &pixel_colors {
            write_color(&mut writer, color)?;
            max_samples_used = max_samples_used.max(samples);
        }

        // --- WRITE TO HEATMAP FILE ---
        if let Some(heatmap_file) = heatmap_file {
            let mut heatmap_writer = BufWriter::new(heatmap_file);
            writeln!(heatmap_writer, "P3")?;
            writeln!(
                heatmap_writer,
                "{} {}",
                camera.resolution.width, camera.resolution.height
            )?;
            writeln!(heatmap_writer, "255")?;

            for &(_, samples) in &pixel_colors {
                let intensity = samples as f64 / max_samples_used as f64;
                write_color(
                    &mut heatmap_writer,
                    Color::new(intensity, intensity, intensity),
                )?;
            }
        }

        pb.finish();
        Ok(())
    }

    pub fn single_threaded_render(
        &self,
        camera: &Camera,
        world: &World,
        file: File,
        heatmap_file: Option<File>,
        style: Option<ProgressStyle>,
    ) -> std::io::Result<()> {
        // --- PROGRESS BAR ---
        let style = style.unwrap_or(
            ProgressStyle::with_template("[{elapsed_precise}] [{bar:40}] {percent:>3}%")
                .unwrap()
                .progress_chars("#>-"),
        );

        let total_pixels = camera.resolution.width * camera.resolution.height;
        let pb = ProgressBar::new(total_pixels as u64);
        pb.set_style(style);

        // --- SETUP WRITER FOR MAIN FILE ---
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        // for writing to heatmap
        let mut samples_used: Vec<i32> = Vec::new();
        let mut max_samples_used = 0;

        // --- MAIN LOOP ---
        for j in 0..camera.resolution.height {
            for i in 0..camera.resolution.width {
                let mut stats = RunningStats::new();

                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for s in 0..self.max_samples {
                    let time = match &self.time_sampler {
                        None => random_unit_f64(),
                        Some(sampler) => sampler(s + i + camera.resolution.width * j),
                    };
                    let ray = camera.sample_ray(i, j, time);
                    let ray_color = ray_color(&ray, self.max_depth, world);
                    pixel_color += ray_color;
                    let luminance =
                        0.2126 * ray_color.x + 0.7152 * ray_color.y + 0.0722 * ray_color.z;
                    // Update stats
                    stats.add(luminance);

                    if stats.n >= Self::MIN_SAMPLES {
                        // standard error
                        let se = (stats.variance() / stats.n as f64).sqrt();
                        if se < Self::TOLERABLE_CV * stats.mean.max(1e-3) {
                            break;
                        }
                    }
                }

                write_color(&mut writer, pixel_color / stats.n as f64)?;
                max_samples_used = max_samples_used.max(stats.n);
                samples_used.push(stats.n);
                pb.inc(1);
            }
        }

        // --- WRITE TO HEATMAP FILE ---
        if let Some(heatmap_file) = heatmap_file {
            let mut heatmap_writer = BufWriter::new(heatmap_file);
            writeln!(heatmap_writer, "P3")?;
            writeln!(
                heatmap_writer,
                "{} {}",
                camera.resolution.width, camera.resolution.height
            )?;
            writeln!(heatmap_writer, "255")?;

            for &samples in &samples_used {
                let intensity = samples as f64 / max_samples_used as f64;
                write_color(
                    &mut heatmap_writer,
                    Color::new(intensity, intensity, intensity),
                )?;
            }
        }

        pb.finish();
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

// Helper struct for adaptive sampling
struct RunningStats {
    n: i32,
    mean: f64,
    m2: f64,
}

impl RunningStats {
    fn new() -> Self {
        Self {
            n: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }

    fn add(&mut self, x: f64) {
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / self.n as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2
    }

    // Welford's online algorithm
    fn variance(&self) -> f64 {
        if self.n > 1 {
            self.m2 / (self.n as f64 - 1.0)
        } else {
            0.0
        }
    }
}
