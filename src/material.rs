use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Vec3, Vec3 as Color, rand_unitvec};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian(pub Color);

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

pub struct Metal(pub Color);

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &mut Color, scattered: &mut Ray) -> bool {
        let reflected: Vec3 = r_in.dir.unit_vector().reflect(rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attentuatn = self.0;
        scattered.dir.dot(rec.normal) > 0.0
    }
}