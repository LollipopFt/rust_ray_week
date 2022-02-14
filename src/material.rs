use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3 as Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attentuatn: &Color, scattered: &Ray) -> bool;
}