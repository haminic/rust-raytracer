use crate::prelude::*;

/*
    This file contains
        1. Camera
        2. Resoltion
*/

pub struct Camera {
    pub resolution: Resolution,

    center: Point3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(position: CameraPosition, resolution: Resolution, settings: CameraSettings) -> Self {
        let center = position.look_from;

        let theta = settings.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * settings.focus_distance;
        let viewport_width = viewport_height * (resolution.width as f64 / resolution.height as f64);

        let w = (position.look_from - position.look_at).unit_vector();
        let u = position.up_direction.cross(w).unit_vector();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = -viewport_height * v;

        let pixel_delta_u = viewport_u / resolution.width as f64;
        let pixel_delta_v = viewport_v / resolution.height as f64;

        let viewport_upper_left =
            center - settings.focus_distance * w - 0.5 * viewport_u - 0.5 * viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * pixel_delta_u + 0.5 * pixel_delta_v;

        let defocus_radius =
            settings.focus_distance * (settings.defocus_angle / 2.0).to_radians().tan();
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

    pub fn sample_ray(&self, i: u32, j: u32, time: f64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;
        let defocus_disk_sample = self.sample_defocus_disk();
        // Monte Carlo
        Ray::with_time(
            defocus_disk_sample,
            pixel_sample - defocus_disk_sample,
            time,
        )
    }

    fn sample_defocus_disk(&self) -> Point3 {
        let p = sample_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}

#[derive(Clone, Copy)]
pub struct CameraPosition {
    pub look_from: Point3,
    pub look_at: Point3,
    pub up_direction: Vec3,
}

#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn with_aspect_ratio(aspect_ratio: f64, width: u32) -> Self {
        let height = ((width as f64 / aspect_ratio) as u32).max(1);
        Self { width, height }
    }

    pub fn square(width: u32) -> Self {
        let height = width;
        Self { width, height }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

#[derive(Clone, Copy)]
pub struct CameraSettings {
    pub vertical_fov: f64,
    pub focus_distance: f64,
    pub defocus_angle: f64,
}

impl CameraSettings {
    pub fn with_fov(vertical_fov: f64) -> Self {
        Self {
            vertical_fov,
            focus_distance: 10.0,
            defocus_angle: 0.0,
        }
    }
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            vertical_fov: 60.0,
            focus_distance: 10.0,
            defocus_angle: 0.0,
        }
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random_unit_f64() - 0.5, random_unit_f64() - 0.5, 0.0)
}

fn sample_in_unit_disk() -> Vec3 {
    use rand::random_range;
    let r = random_unit_f64().sqrt();
    let theta = random_range(0.0..(2.0 * PI));
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3::new(r * cos_theta, r * sin_theta, 0.0)
}
