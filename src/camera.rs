use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub width: Vec3,
    pub height: Vec3,
}

impl Camera {
    pub fn new(origin: &Vec3, lower_left_corner: &Vec3, width: &Vec3, height: &Vec3) -> Self {
        Self {
            origin: origin.clone(),
            lower_left_corner: lower_left_corner.clone(),
            width: width.clone(),
            height: height.clone(),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(&(&self.lower_left_corner + &(u * self.width) + (v * self.height)) - &self.origin),
        )
    }
}

#[cfg(test)]
mod camera_tests {
    #[test]
    fn test() {
        use super::*;

        let orig = Vec3::new(0.0, 0.0, 0.0);
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let width = Vec3::new(2.0, 0.0, 0.0);
        let height = Vec3::new(0.0, 2.0, 0.0);
        let cam = Camera::new(&orig, &lower_left_corner, &width, &height);

        assert_eq!(cam.origin, orig);
        assert_eq!(
            cam.get_ray(0.0, 1.0),
            Ray::new(&orig, &Vec3::new(-2.0, 1.0, -1.0))
        );
    }
}
