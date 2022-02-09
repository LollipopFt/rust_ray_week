use crate::vec3::Vec3 as Color;

pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}",
        (pixel_color.0*255.999) as u8,
        (pixel_color.1*255.999) as u8,
        (pixel_color.2*255.999) as u8
    )
}