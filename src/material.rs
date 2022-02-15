use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Vec3, Vec3 as Color, rand_unitvec};


#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color)
}

impl Default for Material {
    fn default() -> Self {Material::Lambertian(Color::default())}
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(c) => {
                let mut scatter_dir = rec.normal + rand_unitvec();
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                }
                *scattered = Ray::new(rec.p, scatter_dir);
                *attentuatn = *c;
                true
            },
            Material::Metal(c) => {
                let reflected: Vec3 = r_in.dir.unit_vector().reflect(rec.normal);
                *scattered = Ray::new(rec.p, reflected);
                *attentuatn = *c;
                scattered.dir.dot(rec.normal) > 0.0
            }
        }
    }
}