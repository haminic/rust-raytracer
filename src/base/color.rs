use std::fs::File;
use std::io::{BufWriter, Write};

use crate::prelude::*;

pub type Color = Vec3;

pub fn write_color(writer: &mut BufWriter<File>, pixel_color: &Color) -> std::io::Result<()> {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;

    writeln!(writer, "{} {} {}", ir, ig, ib)
}
