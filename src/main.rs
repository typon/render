#![allow(dead_code)]

use rand::Rng;
use std::rc::Rc;

mod camera;
mod material;
mod objects;
mod ray;
mod vec3;
use crate::camera::Camera;
use crate::material::{Lambertian, Material, Metal};
use crate::objects::{random_point_in_unit_sphere, Collection, Object, Sphere};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

fn get_color(ray: &Ray, world: &dyn Object, depth: u16) -> Vec3 {
    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(hit_rec) => {
            if depth > 50 {
                return Vec3::zeros();
            }
            match hit_rec.material.upgrade().unwrap().scatter(&ray, &hit_rec) {
                Some((scattered_ray, attenuation)) => {
                    return attenuation * get_color(&scattered_ray, world, depth + 1);
                }
                None => {
                    return Vec3::zeros();
                }
            }
        }
        None => {
            let unit_direction: Vec3 = unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::ones() + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    const NUM_COLS: usize = 600;
    const NUM_ROWS: usize = 300;
    const NUM_SAMPLES: usize = 300;
    println!("P3");
    println!("{} {}", NUM_COLS, NUM_ROWS,);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let width = Vec3::new(4.0, 0.0, 0.0);
    let height = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zeros();
    let cam = Camera::new(&origin, &lower_left_corner, &width, &height);
    let world = Collection {
        objs: vec![
            Box::new(Sphere::new(
                &Vec3::new(0.0, 0.0, -1.0),
                0.5,
                Rc::new(Box::new(Lambertian {
                    albedo: Vec3::new(0.8, 0.3, 0.3),
                }) as Box<dyn Material>),
            )),
            Box::new(Sphere::new(
                &Vec3::new(0.0, -100.5, -1.0),
                100.0,
                Rc::new(Box::new(Lambertian {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                }) as Box<dyn Material>),
            )),
            Box::new(Sphere::new(
                &Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Rc::new(Box::new(Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                }) as Box<dyn Material>),
            )),
            Box::new(Sphere::new(
                &Vec3::new(-1.0, 0.0, -1.2),
                0.5,
                Rc::new(Box::new(Metal {
                    albedo: Vec3::new(0.8, 0.8, 0.8),
                }) as Box<dyn Material>),
            )),
        ],
    };

    for j in (0..NUM_ROWS).rev() {
        for i in 0..NUM_COLS {
            let mut color = Vec3::zeros();
            for _s in 0..NUM_SAMPLES {
                let u = (i as f32 + rng.gen::<f32>()) / NUM_COLS as f32;
                let v = (j as f32 + rng.gen::<f32>()) / NUM_ROWS as f32;
                let ray = cam.get_ray(u, v);
                color += get_color(&ray, &world, 0);
            }
            color /= NUM_SAMPLES as f32;
            color = Vec3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

            let ir: u32 = (255.99f32 * color.r()) as u32;
            let ig: u32 = (255.99f32 * color.g()) as u32;
            let ib: u32 = (255.99f32 * color.b()) as u32;
            println!("{ir} {ig} {ib}", ir = ir, ig = ig, ib = ib);
        }
    }

    // let v2 = Vec3::new(1.0, 2.0, 4.0);
    // println!("{}", &v + &v2);
    // println!("{}", v[2]);
    // println!("{}", -&v);
}
