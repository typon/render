use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use rand::Rng;
use std::rc::{Rc, Weak};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Weak<Box<dyn Material>>,
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Collection {
    pub objs: Vec<Box<dyn Object>>,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32, material: Rc<Box<dyn Material>>) -> Self {
        Self {
            center: center.clone(),
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let close_t = (-b - discriminant.sqrt()) / a;
            // Hit happens in allowed range
            if close_t < t_max && close_t > t_min {
                let p = ray.point_at_param(close_t);
                let normal = &(p - self.center) / self.radius;
                return Some(HitRecord {
                    t: close_t,
                    p,
                    normal,
                    material: Rc::downgrade(&self.material),
                });
            }
            let far_t = (-b + discriminant.sqrt()) / a;
            if far_t < t_max && far_t > t_min {
                let p = ray.point_at_param(far_t);
                let normal = &(p - self.center) / self.radius;
                return Some(HitRecord {
                    t: far_t,
                    p,
                    normal,
                    material: Rc::downgrade(&self.material),
                });
            }
        }
        return None;
    }
}

impl Object for Collection {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for obj in &self.objs {
            match obj.hit(ray, t_min, closest_so_far) {
                Some(hit_rec) => {
                    closest_so_far = hit_rec.t;
                    closest_hit_rec = Some(hit_rec);
                }
                None => {}
            }
        }
        return closest_hit_rec;
    }
}

pub fn random_point_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p =
            2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - Vec3::ones();
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod objects_tests {
    #[test]
    fn sphere() {
        use super::*;
        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let lamb = Rc::new(Box::new(Lambertian { albedo }) as Box<dyn Material>);
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(&orig, 2.0, lamb);
        assert_eq!(sphere.radius, 2.0);
        assert_eq!(sphere.center, orig);
    }
}
