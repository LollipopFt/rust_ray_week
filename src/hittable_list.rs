use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

struct HitList(Vec<Box<dyn Hittable>>);

impl Hittable for HitList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything: bool = false;
        let mut closest_sofar = t_max;
        for object in self.0.iter() {
            if object.hit(r, t_min, closest_sofar, rec) {
                hit_anything = true;
                closest_sofar = rec.t;
            }
        }

        hit_anything
    }
}