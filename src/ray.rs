use crate::vec3::Vec3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {orig, dir}
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.orig + t*self.dir
    }
}