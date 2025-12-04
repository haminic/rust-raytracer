mod vec3;

use std::fs::File;
use std::io::{BufWriter, Write, stdout};

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        let progress = j as f64 / (image_height - 1) as f64;
        show_progress(progress);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.0 * r) as i32;
            let ig = (255.0 * g) as i32;
            let ib = (255.0 * b) as i32;

            writeln!(writer, "{} {} {}", ir, ig, ib)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

fn show_progress(progress: f64) {
    let bar_width = 25;
    let filled = (progress * bar_width as f64) as usize;

    let bar = format!(
        "[{}{}] {:3}%",
        "=".repeat(filled),
        " ".repeat(bar_width - filled),
        (progress * 100.0) as i32
    );

    print!("\r{}", bar);
    stdout().flush().unwrap();
}
