use crate::vec3::{self, Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at_t(self, t: f64) -> Point3 {
        self.origin + self.direction * t
   } 
}
