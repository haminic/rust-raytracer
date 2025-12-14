use std::fs::File;
use std::io::{BufWriter, Write};

use crate::base::{Interval, Vec3};

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: Color) -> std::io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let r_byte = (256.0 * intensity.clamp(r)) as i32;
    let g_byte = (256.0 * intensity.clamp(g)) as i32;
    let b_byte = (256.0 * intensity.clamp(b)) as i32;

    writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
}

pub fn luminance(color: Color) -> f64 {
    0.2126 * color.x + 0.7152 * color.y + 0.0722 * color.z
}

pub fn get_heatmap_color(value: f64) -> Color {
    let v = value.clamp(0.0, 1.0);
    let r;
    let g;
    let b;

    if v <= 0.25 {
        // Blue to Cyan
        r = 0.0;
        g = 4.0 * v;
        b = 1.0;
    } else if v <= 0.5 {
        // Cyan to Green
        r = 0.0;
        g = 1.0;
        b = 1.0 - 4.0 * (v - 0.25);
    } else if v <= 0.75 {
        // Green to Yellow
        r = 4.0 * (v - 0.5);
        g = 1.0;
        b = 0.0;
    } else {
        // Yellow to Red
        r = 1.0;
        g = 1.0 - 4.0 * (v - 0.75);
        b = 0.0;
    }

    Color::new(r, g, b)
}
