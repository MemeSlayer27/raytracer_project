use std::{rc::Rc, clone, cell::RefCell, ops::Deref, borrow::BorrowMut};

use crate::{vec3::{Point3, Vec3, dot}, ray, material::{Material, self}};

use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub r: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(r.direction, oc);
        let c = oc.length_squared() - self.r * self.r;

        let discriminant = half_b * half_b - a * c;

        
        if discriminant < 0.0 {
            return false
        } 
       
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root<t_min || t_max<root {
            root = (-half_b + sqrtd) / a;
            if root<t_min || t_max<root {
                return false
            } 
        }

        rec.t = root;
        rec.p = r.at_t(rec.t);
        
        let outward_normal = (rec.p - self.center) / self.r;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
