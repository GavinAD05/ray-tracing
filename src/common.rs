use rand::RngExt;

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Random float [0, 1)
pub fn random_f64() -> f64 {
    rand::rng().random_range(0.0..1.0)
}

// Random float [min, max)
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::rng().random_range(min..max)
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
