use std::fs::File;
use std::io::{BufWriter, Write};

use crate::prelude::*;

pub type Color = Vec3;

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: &Color) -> std::io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let intensity = Interval::new(0.000, 0.999);
    let r_byte = (256.0 * intensity.clamp(r)) as i32;
    let g_byte = (256.0 * intensity.clamp(g)) as i32;
    let b_byte = (256.0 * intensity.clamp(b)) as i32;

    writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
}
