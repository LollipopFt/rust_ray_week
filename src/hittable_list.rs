use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct HitList(pub Vec<Box<dyn Hittable>>);

impl HitList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

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