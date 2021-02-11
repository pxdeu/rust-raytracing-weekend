use std::f64::consts::PI;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::write_color;
use crate::dielectric::Dielectric;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::*;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::metal::Metal;
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
mod material;
mod lambertian;
mod metal;
mod dielectric;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return BLACK;
    }

    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        return if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            BLACK
        };
    }

    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    WHITE * (1.0 - t) + LIGHT_BLUE * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let sphere = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(ground_material),
    );
    world.add(sphere);

    let some_point = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random();
            let center = Point3::new(a as f64 + 0.9 * util::random(), 0.2, b as f64 + 0.9 * util::random());

            if (center - some_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(sphere);
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_between(0.5, 1.0);
                    let fuzz = util::random_between(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(sphere);
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(center, 0.2, Arc::new(sphere_material));
                    world.add(sphere);
                }
            }
        }
    }

    let sphere_material = Dielectric::new(1.5);
    let sphere = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(sphere);

    let sphere_material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(sphere);

    let sphere_material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(sphere_material));
    world.add(sphere);

    world
}

fn main() {

    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: i32 = 1200;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene(); //HittableList::new();

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus =  10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
