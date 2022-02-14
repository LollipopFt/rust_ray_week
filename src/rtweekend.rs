use rand::{Rng, thread_rng};

fn degree_toradian(degree: f64) -> f64 {
    degree*std::f64::consts::PI / 180.0
}

pub fn random() -> f64 {
    thread_rng().gen::<f64>()
}

pub fn rand_double(min: f64, max: f64) -> f64 {
    min + (max-min)*random()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}