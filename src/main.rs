use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::*;
use crate::ray::*;
use crate::sphere::Sphere;
use crate::util::random;
use crate::vec3::*;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod util;
mod camera;
mod color;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return BLACK;
    }

    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_unit_vector();
        let bounce_ray = Ray::new(rec.p, target - rec.p);
        return ray_color(&bounce_ray, world, depth - 1) * 0.5;
    }
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    WHITE * (1.0 - t) + LIGHT_BLUE * t
}

fn main() {

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

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
                pixel_color += ray_color(&r, &world, max_depth)
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done.")
}
