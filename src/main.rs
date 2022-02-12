use std::io::Write;

mod vec3;
use vec3::Vec3;
use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;
mod color;
use color::write_color;
mod ray;
use ray::Ray;
mod hittable;
use hittable::{Hittable, HitRecord};
mod hittable_list;
use hittable_list::HitList;
mod sphere;
use sphere::Sphere;

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = Default::default();
    if world.hit(&r, 0.0, std::f64::INFINITY, &mut rec) {
        0.5*(rec.normal+Color(1.0, 1.0, 1.0))
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

    // world:
    let mut world: HitList = HitList(vec![]);
    world.add(Box::new(Sphere {
        cen: Point3(0.0, 0.0, -1.0),
        r: 0.5
    }));
    world.add(Box::new(Sphere {
        cen: Point3(0.0, -100.5, -1.0),
        r: 100.0
    }));

    // camera:
    const VIEWPORTHEIGHT: f64 = 2.0;
    const VIEWPORTWIDTH: f64 = ASPECTRATIO*VIEWPORTHEIGHT;
    const FOCALLENGTH: f64 = 1.0;

    let origin: Point3 = Default::default();
    let horizontal = Vec3(VIEWPORTWIDTH, 0.0, 0.0);
    let vertical = Vec3(0.0, VIEWPORTHEIGHT, 0.0);
    let lowleft_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3(0.0, 0.0, FOCALLENGTH);

    // render:
    let mut handle = std::io::BufWriter::new(std::io::stdout());
    writeln!(handle, "P3\n{IMAGEWIDTH} {IMAGEHEIGHT}\n255").ok();
    for j in (0..IMAGEHEIGHT).rev() {
        eprint!("scanlines remaining: {j}\r");
        for i in 0..IMAGEWIDTH {
            let u = i as f64 / (IMAGEWIDTH-1) as f64;
            let v = j as f64 / (IMAGEHEIGHT-1) as f64;
            let r = Ray::new(origin, lowleft_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r, &world);
            writeln!(handle, "{}", write_color(pixel_color)).ok();
        }
    }
    eprint!("\ndone.");
}