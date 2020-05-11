#![allow(dead_code)]

mod ray;
mod vec3;
// use std::{thread, time};
use crate::vec3::{dot, Vec3};

fn main() {
    const NUM_COLS: usize = 200;
    const NUM_ROWS: usize = 100;
    println!("P3");
    println!("{} {}", NUM_COLS, NUM_ROWS,);
    println!("255");

    for j in (0..NUM_ROWS).rev() {
        for i in 0..NUM_COLS {
            let r: f32 = i as f32 / NUM_COLS as f32;
            let g: f32 = j as f32 / NUM_ROWS as f32;
            let b: f32 = 0.2;
            let ir: u32 = (256f32 * r) as u32;
            let ig: u32 = (256f32 * g) as u32;
            let ib: u32 = (256f32 * b) as u32;
            // println!("{ir} {ig} {ib}", ir = ir, ig = ig, ib = ib);
        }
    }

    let v2 = Vec3::new(1.0, 2.0, 4.0);
    println!("{}", &v + &v2);
    println!("{}", v[2]);
    println!("{}", -&v);
}
