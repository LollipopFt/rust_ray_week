use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point3;
use crate::ray::Ray;

#[derive(Default)]
pub struct Camera {
    orig: Point3,
    lowleft_corner: Point3,
    horiz: Vec3,
    vert: Vec3
}

impl Camera {
    pub fn camera(&mut self) {
        const ASPECTRATIO: f64 = 16.0/9.0;
        const VIEWPORTHEIGHT: f64 = 2.0;
        const VIEWPORTWIDTH: f64 = ASPECTRATIO*VIEWPORTHEIGHT;
        const FOCALLENGTH: f64 = 1.0;

        self.orig = Default::default();
        self.horiz = Vec3(VIEWPORTWIDTH, 0.0, 0.0);
        self.vert = Vec3(0.0, VIEWPORTHEIGHT, 0.0);
        self.lowleft_corner = self.orig - self.horiz/2.0 - self.vert/2.0 - Vec3(0.0, 0.0, FOCALLENGTH);
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.lowleft_corner + u*self.horiz + v*self.vert - self.orig
        }
    }
}