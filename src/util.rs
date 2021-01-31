use rand::{thread_rng, Rng};
use std::cmp;

pub fn random() -> f64 {
    thread_rng().gen()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
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