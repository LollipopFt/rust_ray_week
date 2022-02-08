fn main() {
    // image:
    const IMAGEWIDTH: u16 = 256;
    const IMAGEHEIGHT: u16 = 256;

    // render:
    println!("P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255");
    for j in (0..IMAGEHEIGHT).rev() {
        eprint!("scanlines remaining: {j}\r");
        for i in 0..IMAGEWIDTH {
            let r = i as f32 / (IMAGEWIDTH-1) as f32;
            let g = j as f32 / (IMAGEHEIGHT-1) as f32;
            let b = 0.25;

            let ir: u8 = (255.999*r) as u8;
            let ig: u8 = (255.999*g) as u8;
            let ib: u8 = (255.999*b) as u8;

            println!("{ir} {ig} {ib}");
        }
    }
    eprint!("\ndone.");
}