use crate::vec3::Vec3 as Point3;
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere {
    pub cen: Point3,
    pub r: f64,
    pub m: Material
}