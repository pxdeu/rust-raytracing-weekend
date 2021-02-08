use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::material::Material;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {

    pub fn new(p: Point3, normal: Vec3, t: f64, material: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
            material
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> ;
}