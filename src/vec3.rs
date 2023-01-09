use std::{ops, process::Output, fmt::{format, Debug}, i64::MIN, cmp::{min_by, min_by_key}};

use rand::Rng;





pub mod color;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64{
       let result =  self.0 * self.0 + self.1 * self.1 + self.2 * self.2;
       result
    }

    fn print_format(&self) -> String {
        let str0 = self.0.to_string();
        let str1 = self.1.to_string();
        let str2 = self.2.to_string();
        
        format!("{} {} {}", str0, str1, str2)

    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    pub fn random_with_limits(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s &&self.2.abs() < s
    }
    
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}


impl ops::MulAssign<f64> for Vec3 {

    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {

    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}


impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}


impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
         self * (1 as f64/rhs)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Vec3 {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)   
    }
}

impl Vec3 {
    pub fn dot(&self, v: Vec3) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    fn cross(&self, v: Vec3) -> Vec3 {
        Vec3(self.1*v.2-self.2*v.1, self.2*v.0-self.0*v.2, self.0*v.1-self.1*v.0)
    }

    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();

        self / len
    
    }

    pub fn random_in_unit_sphere() -> Point3 {
        loop {
            let p = Vec3::random_with_limits( -1.0, 1.0);
            if p.length_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }
    
    pub fn random_in_unit_disk() -> Point3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length() > 1.0 { continue; }
            return p;
        }
    }
    



}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta: f64 = dot(-uv,n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.0 * v.0 + u.1 * v.1 + u.2 * v.2
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.1*v.2-u.2*v.1, u.2*v.0-u.0*v.2, u.0*v.1-u.1*v.0)
}

pub type Point3 = Vec3;
pub type Color = Vec3;