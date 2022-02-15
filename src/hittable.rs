use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point3;
use crate::material::Material;
use crate::sphere;
use crate::hittable_list;

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn setface_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub enum Hittable {
    Sphere(sphere::Sphere),
    HitList(hittable_list::HitList)
}

impl Hittable {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        match self {
            Hittable::Sphere(color) => {
                let oc: Vec3 = r.orig - color.cen;
                let a = r.dir.length_squared();
                let half_b = oc.dot(r.dir);
                let c = oc.length_squared() - color.r*color.r;

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
                let outward_normal = (rec.p - color.cen) / color.r;
                rec.setface_normal(*r, outward_normal);
                rec.material = color.m;

                true
            },

            Hittable::HitList(hitlist) => {
                let mut hit_anything: bool = false;
                let mut closest_sofar = t_max;
                for object in hitlist.0.iter() {
                    if object.hit(r, t_min, closest_sofar, rec) {
                        hit_anything = true;
                        closest_sofar = rec.t;
                    }
                }

                hit_anything
            }
        }
    }

    pub fn add(&mut self, object: Hittable) {
        if let Hittable::HitList(hitlist) = self {
            if let Hittable::Sphere(_) = object {
                hitlist.0.push(object);
            }
        }
    }

    pub fn clear(&mut self) {
        if let Hittable::HitList(hitlist) = self {
            hitlist.0.clear()
        }
    }
}