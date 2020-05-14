use crate::objects::{random_point_in_unit_sphere, HitRecord};
use crate::ray::Ray;
use crate::vec3::{dot, unit_vector, Vec3};
use std::rc::Rc;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit_rec.p + hit_rec.normal + random_point_in_unit_sphere();
        let scattered_ray = Ray::new(&hit_rec.p, &(target - hit_rec.p));
        let attenuation = &self.albedo;
        return Some((scattered_ray, attenuation.clone()));
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&unit_vector(ray.direction()), &hit_rec.normal);
        let scattered_ray = Ray::new(&hit_rec.p, &reflected);
        let attenuation = &self.albedo;
        if dot(scattered_ray.direction(), &hit_rec.normal) > 0.0 {
            Some((scattered_ray, attenuation.clone()))
        } else {
            None
        }
    }
}
// Reflected ray = v - 2B
// B = dot(v, normal)
//          normal
//  X      ^      X
//   X     |     X ^
//    X    |    X  |
//     X   |   X   |
//  v   X  |  X    |  B
//       X | X     |
//        X|X      |
//         |       |
//+--------+----------------+
//          X      ^
//           X     |
//            X    |
//             X   |  B
//         v    X  |
//               X +
//                X
//

fn reflect(v: &Vec3, normal: &Vec3) -> Vec3 {
    return v - &(normal * 2.0 * dot(v, normal));
}

#[cfg(test)]
mod material_tests {
    #[test]
    fn test() {
        use super::*;

        let albedo = Vec3::new(0.8, 0.6, 0.2);
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 1.0, 1.0);
        let l = Rc::new(Box::new(Lambertian { albedo }) as Box<dyn Material>);
        let hit_rec = HitRecord {
            t: 1.0,
            p: dir.clone(),
            normal: dir.clone(),
            material: Rc::downgrade(&l),
        };
        let r = Ray::new(&orig, &dir);

        let (scattered_ray, attenuation) = l.scatter(&r, &hit_rec).unwrap();
        assert_eq!(attenuation, albedo);
    }
}
