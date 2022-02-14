use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point3;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    pub cen: Point3,
    pub r: f64,
    pub m: Box<dyn Material>
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.cen;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.r*self.r;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.cen) / self.r;
        rec.setface_normal(*r, outward_normal);
        rec.material = self.m;

        true
    }
}