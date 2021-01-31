use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                record.t = temp_rec.t;
                record.p = temp_rec.p;
                record.front_face = temp_rec.front_face;
                record.normal = temp_rec.normal;
            }
        }

        hit_anything
    }
}