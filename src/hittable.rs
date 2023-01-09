use std::{str::MatchIndices, rc::Rc, cell::{RefCell, RefMut},};

use crate::{vec3::{Point3, Vec3, dot}, ray::Ray, material::Material};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,

}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.0;
        self.normal = match self.front_face {
            true  => outward_normal,
            false => -(outward_normal),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}