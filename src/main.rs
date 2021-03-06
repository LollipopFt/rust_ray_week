use std::io::Write;

mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod camera;
mod material;

use vec3::Vec3;
use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;

fn ray_color(r: ray::Ray, world: &hittable::Hittable, depth: u16) -> Color {
    let mut rec: hittable::HitRecord = Default::default();
    if depth == 0 {
        Color::default()
    } else if world.hit(&r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: ray::Ray = Default::default();
        let mut attentuatn: Color = Default::default();
        if rec.material.scatter(&r, &rec, &mut attentuatn, &mut scattered) {
            attentuatn*ray_color(scattered, world, depth-1)
        } else {
            Color::default()
        }
    } else {
        let unit_dir: Vec3 = r.dir.unit_vector();
        let t = 0.5*(unit_dir.1 + 1.0);
        (1.0-t)*Color(1.0, 1.0, 1.0) + t*Color(0.5, 0.7, 1.0)
    }
}

fn main() {
    // image:
    const ASPECTRATIO: f64 = 16.0/9.0;
    const IMAGEWIDTH: u16 = 400;
    const IMAGEHEIGHT: u16 = (IMAGEWIDTH as f64/ASPECTRATIO) as u16;
    const SAMPLESPERPIXEL: u16 = 100;
    const MAXDEPTH: u16 = 50;

    // world:
    let mut world: hittable::Hittable = hittable::Hittable::HitList(hittable_list::HitList(vec![]));
    let material_ground = material::Material::Lambertian(Color(0.8, 0.8, 0.0));
    let material_center = material::Material::Lambertian(Color(0.7, 0.3, 0.3));
    let material_left = material::Material::Metal(Color(0.8, 0.8, 0.8));
    let material_right = material::Material::Metal(Color(0.8, 0.6, 0.2));

    world.add(hittable::Hittable::Sphere(
        sphere::Sphere {
            cen: Point3(0.0, -100.5, -1.0),
            r: 100.0,
            m: material_ground
        }
    ));
    world.add(hittable::Hittable::Sphere(
        sphere::Sphere {
        cen: Point3(0.0, 0.0, -1.0),
        r: 0.5,
        m: material_center
        }
    ));
    world.add(hittable::Hittable::Sphere(
        sphere::Sphere {
        cen: Point3(-1.0, 0.0, -1.0),
        r: 0.5,
        m: material_left
        }
    ));
    world.add(hittable::Hittable::Sphere(
        sphere::Sphere {
        cen: Point3(1.0, 0.0, -1.0),
        r: 0.5,
        m: material_right
        }
    ));

    // camera:
    let mut cam = camera::Camera::default();
    cam.camera();

    // render:
    let mut handle = std::io::BufWriter::new(std::io::stdout());
    writeln!(handle, "P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255").ok();
    for j in (0..IMAGEHEIGHT).rev() {
        eprint!("scanlines remaining: {j}\u{9B}K\r");
        for i in 0..IMAGEWIDTH {
            let mut pixel_color: Color = Default::default();
            for _ in 0..SAMPLESPERPIXEL {
                let u = (i as f64+rtweekend::random())/(IMAGEWIDTH-1) as f64;
                let v = (j as f64+rtweekend::random())/(IMAGEHEIGHT-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAXDEPTH);
            }
            writeln!(handle, "{}", color::write_color(pixel_color, SAMPLESPERPIXEL)).ok();
        }
    }
    eprint!("\ndone.");
}