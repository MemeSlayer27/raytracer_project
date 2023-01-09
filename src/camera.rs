use crate::{vec3::{Vec3, Point3, cross}, ray::Ray, usefulstuff::degrees_to_radians};



#[derive(Clone, Copy)]
pub struct Camera {
    pub vfov: f64,
    pub aspect_ratio: f64,
    pub look_at: Point3,
    pub look_from: Point3,
    pub vup: Vec3,
    pub aperture: f64,
    pub focus_distace: f64,


}

impl Camera {

    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.look_from - self.look_at).unit_vector();
        let u = cross(self.vup,w).unit_vector();
        let v = cross(w,u);


        let origin = self.look_from;
        let horizontal = self.focus_distace * viewport_width * u;
        let vertical = self.focus_distace * viewport_height * v;

        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - self.focus_distace * w;

        let lens_radius = self.aperture / 2.0;

        let rd = lens_radius * Vec3::random_in_unit_disk();
        let offset = u * rd.x() + v * rd.y();

        Ray { origin: origin + offset, direction: lower_left_corner + s*horizontal + t*vertical - origin - offset }
    }
}



