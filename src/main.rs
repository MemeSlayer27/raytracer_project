use std::{f64::INFINITY, rc::Rc, };
use rand::Rng;

use hittable::HitRecord;
use sphere::Sphere;

use crate::{vec3::{Vec3, Color, Point3, color::{write_color}}, material::{Lambertian, Metal, Dielectric}};
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::camera::Camera;

pub mod vec3; 
pub mod ray;
pub mod sphere;
pub mod usefulstuff;
pub mod hittable;
pub mod hittable_list;
pub mod camera;
pub mod material;

fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec: HitRecord = HitRecord {            // this should never get used as is. All hits hould modify this
        p: Vec3(0.0,0.0,0.0), 
        normal: Vec3(0.0,0.0,0.0), 
        t: 0.0, front_face: true, 
        mat_ptr: Rc::new(Lambertian{albedo: Vec3(0.0,0.0,0.0) as Color})
    };

    if depth <= 0 { return Vec3(0.0, 0.0, 0.0) as Color; }

    if world.hit(&ray, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray{origin: Vec3(0.0,0.0,0.0), direction: Vec3(0.0,0.0,0.0)};
        let mut attenuation: Color = Vec3(0.0,0.0,0.0) as Color;
        if rec.mat_ptr.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
            let color =  attenuation * ray_color(scattered, world, depth-1);
            return color;
        }
        return Vec3(0.0,0.0,0.0) as Color
    }

    let unit_direction: Vec3 = ray.direction.unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    
    let color1 = Vec3(1.0, 1.0, 1.0) as Color;
    let color2 = Vec3(0.5, 0.7, 1.0) as Color;

    (1.0-t)*color1 + t*color2
}

fn random_world() -> HittableList {
    let mut world:HittableList = HittableList { hittables: vec![] };

    let ground_material = Rc::new(Lambertian{albedo: Vec3(0.5,0.5,0.5) as Color});
    world.add(Box::new(Sphere{center: Vec3(0.0, -1000.0, 0.0) as Point3, r: 1000.0, mat_ptr: ground_material}));

    let mut rng = rand::thread_rng();


    for a in -11..11 {
        for b in -11..11 {
            let r = rng.gen::<f64>();

            let choose_mat = rng.gen::<f64>();
            let center = Vec3(a as f64 + 0.9*rng.gen::<f64>(), r, b as f64 + 0.9*rng.gen::<f64>()); //

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian{albedo});
                    world.add(Box::new(Sphere{center, r, mat_ptr: sphere_material}));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_limits(0.5, 1.0);
                    let fuzz = rng.gen::<f64>() / 2.0;
                    let sphere_material = Rc::new(Metal{albedo, fuzz});
                    world.add(Box::new(Sphere{center, r, mat_ptr: sphere_material}));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric{ir: 1.5});
                    world.add(Box::new(Sphere{center, r, mat_ptr: sphere_material}));
                }

            }


        }
    }



    // create a metal material with albedo (0.7, 0.6, 0.5) and no fuzz
    let material3 = Rc::new(Metal {
        albedo: Vec3(0.7, 0.6, 0.5) as Color,
        fuzz: 0.0,
    });


    world.add(Box::new(Sphere {
        center: Vec3(0.0, 1.0, 0.0),
        r: 1.0,
        mat_ptr: material3,
    }));

    // create a lambertian material with albedo (0.4, 0.2, 0.1)
    let material2 = Rc::new(Lambertian {
        albedo: Vec3(0.4, 0.2, 0.1) as Color,
    });
    world.add(Box::new(Sphere {
        center: Vec3(-4.0, 1.0, 0.0),
        r: 1.0,
        mat_ptr: material2,
    }));

    // create a dielectric material with refractive index 1.5
    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Box::new(Sphere {
        center: Vec3(4.0, 1.0, 0.0),
        r: 1.0,
        mat_ptr: material1,
    }));

    world
}

fn main() {

    // Image
    let aspect_ratio = 3.0/2.0;
    let image_width = 1200;  //used to be 400
    let image_height = (image_width as f64 / aspect_ratio).round() as i32;
    let samples_per_pixel = 50.0; // originally 100
    let max_depth = 50;

    //World
    let world = random_world();


    let cam = Camera {
        look_from: Vec3(30.0,10.0,3.0) as Point3,
        look_at: Vec3(4.0, 1.0, 0.0) as Point3, 
        vup: Vec3(0.0,1.0,0.0), 
        vfov: 20.0, 
        aspect_ratio,
        focus_distace: (Vec3(4.0, 1.0, 0.0) - Vec3(20.0,10.0,3.0)).length(),
        aperture: 0.1,
    };

    // Random
    let mut rng = rand::thread_rng();

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines Remaining: {}\n", j);
        for i in 0..image_width {
            let mut pixel_color = Vec3(0.0,0.0,0.0) as Color;
            for _s in 0..(samples_per_pixel as i32) {
                // These rng methods might just be the problem
                let u = (rng.gen::<f64>() - 0.5 + i as f64) / (image_width-1) as f64;
                let v = (rng.gen::<f64>() - 0.5 + j as f64) / (image_height-1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprint!("\nDone!\n");
}

