use super::*;

/*
    This file contains
        1. Camera
        2. Resoltion
*/

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
    pub resolution: Resolution,

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
        let time = random_unit_f64();
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
