use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
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
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32) -> Self {
        Self {
            center: center.clone(),
            radius,
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

#[cfg(test)]
mod objects_tests {
    #[test]
    fn sphere() {
        use super::*;
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let sphere = Sphere::new(&orig, 2.0);
        assert_eq!(sphere.radius, 2.0);
        assert_eq!(sphere.center, orig);
    }
}
