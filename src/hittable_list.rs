use std::rc::Rc;

use crate::vec3::Color;
use crate::{vec3::Vec3, ray::Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Material, Lambertian};

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
}



impl HittableList {
    pub fn add(&mut self, item:Box<dyn Hittable>) {
        self.hittables.push(item);
    }

    //TODO: add the erase method if necessary. Rust might be able to inherently handle it.
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {            // this should never get used as is. All hits hould modify this
            p: Vec3(0.0,0.0,0.0), 
            normal: Vec3(0.0,0.0,0.0), 
            t: 0.0, front_face: true, 
            mat_ptr: Rc::new(Lambertian{albedo: Vec3(0.0,0.0,0.0) as Color})
        };
        
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.hittables {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;

                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.mat_ptr = temp_rec.mat_ptr.clone();

            }
        }
        hit_anything
    }
}
