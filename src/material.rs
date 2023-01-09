use std::ops::DerefMut;

use rand::{random, Rng};

use crate::{hittable::HitRecord, ray::Ray, vec3::{Color, color, Vec3, reflect, dot, refract}};




pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, mut attenuation: &mut Color, mut scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();


        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        attenuation.0 = *&self.albedo.0;
        attenuation.1 = *&self.albedo.1;
        attenuation.2 = *&self.albedo.2;

        //eprintln!("{:?}", &self.albedo);


        //eprint!("{:?}", attenuation);

        scattered.origin = rec.p;
        scattered.direction = scatter_direction;

        //std::mem::swap(scattered,  &mut Ray{origin: rec.p, direction: scatter_direction});
        //std::mem::swap(attenuation,  &mut self.albedo);

        true
    }
}



pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, mut attenuation: &mut Color, mut scattered: &mut Ray) -> bool{
        let reflected = reflect(r_in.direction, rec.normal);

        attenuation.0 = *&self.albedo.0;
        attenuation.1 = *&self.albedo.1;
        attenuation.2 = *&self.albedo.2;


        scattered.origin = rec.p;
        scattered.direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();

        dot(scattered.direction, rec.normal) > 0.0
    }
}


pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0*r0;
        r0 + (1.0-r0)*f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, mut attenuation: &mut Color, mut scattered: &mut Ray) -> bool {
        
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        attenuation.0 = 1.0;
        attenuation.1 = 1.0;
        attenuation.2 = 1.0;

        let unit_direction = Vec3::unit_vector(r_in.direction);

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = sin_theta * refraction_ratio > 1.0;
        let mut rng = rand::thread_rng();

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };


        scattered.origin = rec.p;
        scattered.direction = direction;

        true
    }
    
} 