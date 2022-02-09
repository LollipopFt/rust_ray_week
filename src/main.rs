use std::io::Write;

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
            let r = i as f64 / (IMAGEWIDTH-1) as f64;
            let g = j as f64 / (IMAGEHEIGHT-1) as f64;
            let b = 0.25;

            let ir: u8 = (255.999*r) as u8;
            let ig: u8 = (255.999*g) as u8;
            let ib: u8 = (255.999*b) as u8;

            writeln!(handle, "{ir} {ig} {ib}").ok();
        }
    }
    eprint!("\ndone.");
}