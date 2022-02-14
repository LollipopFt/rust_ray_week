use crate::vec3::Vec3 as Color;
use crate::rtweekend::clamp;

pub fn write_color(pixel_color: Color, samples_perpixel: u16) -> String {
    let mut r = pixel_color.0;
    let mut g = pixel_color.1;
    let mut b = pixel_color.2;

    // divide color by number of samples
    let scale = 1.0/samples_perpixel as f64;
    r = (scale*r).sqrt();
    g = (scale*g).sqrt();
    b = (scale*b).sqrt();

    // write translated [0, 255] value of each color component
    format!(
        "{} {} {}",
        (256.0*clamp(r, 0.0, 0.999)) as u8,
        (256.0*clamp(g, 0.0, 0.999)) as u8,
        (256.0*clamp(b, 0.0, 0.999)) as u8
    )
}