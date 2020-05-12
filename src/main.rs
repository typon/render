#![allow(dead_code)]

mod objects;
mod ray;
mod vec3;
use crate::objects::{Collection, Object, Sphere};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

fn get_color(ray: &Ray, world: &dyn Object) -> Vec3 {
    match world.hit(ray, 0.0, std::f32::MAX) {
        Some(hit_rec) => {
            0.5 * Vec3::new(
                hit_rec.normal.x() + 1.0,
                hit_rec.normal.y() + 1.0,
                hit_rec.normal.z() + 1.0,
            )
        }
        None => {
            let unit_direction: Vec3 = unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const NUM_COLS: usize = 200;
    const NUM_ROWS: usize = 100;
    println!("P3");
    println!("{} {}", NUM_COLS, NUM_ROWS,);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world = Collection {
        objs: vec![
            Box::new(Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    for j in (0..NUM_ROWS).rev() {
        for i in 0..NUM_COLS {
            let u = i as f32 / NUM_COLS as f32;
            let v = j as f32 / NUM_ROWS as f32;

            let ray = Ray::new(
                &origin,
                &(lower_left_corner + (u * horizontal) + (v * vertical)),
            );
            // let p = ray.point_at_param(2.0);
            let color = get_color(&ray, &world);

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
