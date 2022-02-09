use crate::vec3::Vec3 as Color;

pub fn write_color(pixel_color: Color) -> String {
    format!(
        "{} {} {}",
        (pixel_color.x()*255.999) as u8,
        (pixel_color.y()*255.999) as u8,
        (pixel_color.z()*255.999) as u8
    )
}