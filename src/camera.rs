use crate::objects::Hittable;
use crate::prelude::*;

pub struct Camera {
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,

    image_width: i32,
    image_height: i32,

    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
    ) -> Self {
        let image_height = ((image_width as f64 / aspect_ratio) as i32).max(1);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = look_from;

        let focal_length = (look_from - look_at).length();
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - focal_length * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

        Self {
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,

            image_width,
            image_height,

            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(
        &self,
        writer: &mut BufWriter<File>,
        world: &dyn Hittable,
    ) -> std::io::Result<()> {
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(writer, "255")?;

        for j in 0..self.image_height {
            // TODO: Show progress bar
            // let progress = j as f64 / (self.image_height - 1) as f64;
            // show_progress(progress);
            for i in 0..self.image_width {
                println!("pixel ({}, {})", i, j);
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += ray_color(&ray, self.max_depth, world)
                }
                write_color(writer, self.pixel_samples_scale * pixel_color)?;
            }
        }
        // println!("\nDone!");
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        Ray::new(self.center, pixel_sample - self.center)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
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
