use crate::vec3::{Color, Vec3};
use crate::util::clamp;

pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) {

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!("{} {} {}",
             (256.0 * clamp(r, 0.0, 0.999)) as i32,
             (256.0 * clamp(g, 0.0, 0.999)) as i32,
             (256.0 * clamp(b, 0.0, 0.999)) as i32,
    )
}