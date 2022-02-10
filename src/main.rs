use std::io::Write;

mod vec3;
use vec3::Vec3;
use vec3::Vec3 as Color;
use vec3::Vec3 as Point3;
mod color;
use color::write_color;
mod ray;
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc: Vec3 = r.orig - center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(r.dir);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(Point3(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3(0.0, 0.0, -1.0)).unit_vector();
        0.5*Color(n.0+1.0, n.1+1.0, n.2+1.0)
    } else {
        let unit_dir: Vec3 = r.dir.unit_vector();
        t = 0.5*(unit_dir.1 + 1.0);
        (1.0-t)*Color(1.0, 1.0, 1.0) + t*Color(0.5, 0.7, 1.0)
    }
}

fn main() {
    // image:
    const ASPECTRATIO: f64 = 16.0/9.0;
    const IMAGEWIDTH: u16 = 400;
    const IMAGEHEIGHT: u16 = (IMAGEWIDTH as f64/ASPECTRATIO) as u16;

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
            let pixel_color = ray_color(r);
            writeln!(handle, "{}", write_color(pixel_color)).ok();
        }
    }
    eprint!("\ndone.");
}