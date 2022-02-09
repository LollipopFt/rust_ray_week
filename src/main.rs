use std::io::Write;

mod vec3;
use vec3::Vec3 as Color;
mod color;
use color::write_color;

fn main() {
    // image:
    const IMAGEWIDTH: u16 = 256;
    const IMAGEHEIGHT: u16 = 256;

    // render:
    let mut handle = std::io::BufWriter::new(std::io::stdout());
    writeln!(handle, "P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255").ok();
    for j in (0..IMAGEHEIGHT).rev() {
        eprint!("scanlines remaining: {j}\r");
        for i in 0..IMAGEWIDTH {
            let pixel_color = Color(
                i as f64/(IMAGEWIDTH-1) as f64,
                j as f64/(IMAGEHEIGHT-1) as f64,
                0.25
            );
            writeln!(handle, "{}", write_color(pixel_color)).ok();
        }
    }
    eprint!("\ndone.");
}