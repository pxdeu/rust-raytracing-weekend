mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;
mod color;

use crate::vec3::*;
use crate::ray::*;
use crate::hittable_list::*;
use crate::sphere::Sphere;
use crate::hittable::{Hittable, HitRecord};
use crate::color::write_color;
use crate::camera::Camera;
use crate::util::random;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {

    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, f64::MAX, &mut rec) {
        return (rec.normal + Color::new(1.0,1.0,1.0)) * 0.5;
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world)
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done.")
}
