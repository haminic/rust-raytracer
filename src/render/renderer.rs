pub use std::fs::File;
pub use std::io::{BufWriter, Write};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::prelude::*;
use crate::render::{Camera, SampleFn, World};

pub struct Renderer {
    pub max_depth: u32,
    pub time_sampler: Option<SampleFn>,
    pub samples_range: (u32, u32),
    pub tolerable_cv: f64,
}

impl Renderer {
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
        let pixel_colors: Vec<(Color, u32)> = (0..camera.resolution.height)
            .into_par_iter()
            .flat_map(|j| {
                let pb = pb.clone();
                (0..camera.resolution.width).into_par_iter().map(move |i| {
                    let (pixel_color, samples) = self.pixel_color(camera, world, i, j);
                    pb.inc(1);
                    (pixel_color, samples)
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
                let intensity = (samples - self.samples_range.0) as f64
                    / (max_samples_used - self.samples_range.0) as f64;
                write_color(&mut heatmap_writer, get_heatmap_color(intensity))?;
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
        let mut samples_used: Vec<u32> = Vec::new();
        let mut max_samples_used = 0;

        // --- MAIN LOOP ---
        for j in 0..camera.resolution.height {
            for i in 0..camera.resolution.width {
                let (pixel_color, samples) = self.pixel_color(camera, world, i, j);
                write_color(&mut writer, pixel_color)?;
                max_samples_used = max_samples_used.max(samples);
                samples_used.push(samples);
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
                let intensity = (samples - self.samples_range.0) as f64
                    / (max_samples_used - self.samples_range.0) as f64;
                write_color(&mut heatmap_writer, get_heatmap_color(intensity))?;
            }
        }

        pb.finish();
        Ok(())
    }

    // Returns the pixel color and number of samples
    pub fn pixel_color(&self, camera: &Camera, world: &World, i: u32, j: u32) -> (Color, u32) {
        let mut stats = RunningStats::new();
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for s in 0..self.samples_range.1 {
            let time = match &self.time_sampler {
                None => random_unit_f64(),
                Some(sampler) => sampler(s + i + j * camera.resolution.width),
            };
            let ray = camera.sample_ray(i, j, time);
            let ray_color = ray_color(&ray, self.max_depth, world);
            pixel_color += ray_color;
            // Update stats
            stats.add(luminance(ray_color));

            if stats.n >= self.samples_range.0 {
                // standard error
                if stats.variance().sqrt() < self.tolerable_cv * stats.mean.max(0.001) {
                    break;
                }
            }
        }
        (pixel_color / stats.n as f64, stats.n)
    }
}

fn ray_color(ray: &Ray, depth: u32, world: &World) -> Color {
    if depth == 0 {
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
    n: u32,
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
