use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Vec3 as Color, rand_unitvec};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &mut Color, scattered: &mut Ray) -> bool;
}

struct Lambertian(Color);

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = rec.normal + rand_unitvec();
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_dir);
        *attentuatn = self.0;
        true
    }
}