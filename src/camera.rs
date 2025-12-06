use crate::objects::Hittable;
use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

impl Resolution {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn with_aspect_ratio(aspect_ratio: f64, width: i32) -> Self {
        let height = ((width as f64 / aspect_ratio) as i32).max(1);
        Self { width, height }
    }
}

pub struct Camera {
    resolution: Resolution,

    center: Point3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        resolution: Resolution,
        look_from: Point3,
        look_at: Point3,
        up_direction: Vec3,
        vertical_fov: f64,
        focus_distance: f64,
        defocus_angle: f64,
    ) -> Self {
        let center = look_from;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (resolution.width as f64 / resolution.height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = up_direction.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / resolution.width as f64;
        let pixel_delta_v = viewport_v / resolution.height as f64;

        let viewport_upper_left = center - focus_distance * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            resolution,

            center,
            defocus_disk_u,
            defocus_disk_v,

            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn sample_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let defocus_disk_sample = self.sample_defocus_disk();
        Ray::new(defocus_disk_sample, pixel_sample - defocus_disk_sample)
    }

    fn sample_defocus_disk(&self) -> Point3 {
        let p = sample_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}

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

    pub fn render(&self, camera: &Camera, world: &dyn Hittable, file: File) -> std::io::Result<()> {
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3")?;
        writeln!(
            writer,
            "{} {}",
            camera.resolution.width, camera.resolution.height
        )?;
        writeln!(writer, "255")?;

        for j in 0..camera.resolution.height {
            // TODO: Show progress bar
            // let progress = j as f64 / (self.image_height - 1) as f64;
            // show_progress(progress);
            for i in 0..camera.resolution.width {
                // println!("pixel ({}, {})", i, j);
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = camera.sample_ray(i, j);
                    pixel_color += ray_color(&ray, self.max_depth, world)
                }
                write_color(&mut writer, self.pixel_samples_scale * pixel_color)?;
            }
        }
        // println!("\nDone!");
        Ok(())
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
}

fn sample_in_unit_disk() -> Vec3 {
    let r = random_range(0.0..1.0_f64).sqrt();
    let theta = random_range(0.0..(2.0 * PI));
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3::new(r * cos_theta, r * sin_theta, 0.0)
}

fn ray_color(ray: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, Interval::new(0.001, INFINITY)) {
        return if let Some(scatter) = hit.mat.scatter(ray, &hit) {
            scatter.attenuation * ray_color(&scatter.ray_out, depth - 1, world)
        } else {
            Color::new(0.0, 0.0, 0.0)
        };
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
