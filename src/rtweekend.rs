use rand::{Rng, thread_rng};

fn degree_toradian(degree: f64) -> f64 {
    degree*std::f64::consts::PI / 180.0
}

fn random() -> f64 {
    thread_rng().gen::<f64>()
}

fn rand_double(min: f64, max: f64) -> f64 {
    min + (max-min)*random()
}