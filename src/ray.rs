use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3) -> Self {
        Ray {
            a: a.clone(),
            b: b.clone(),
        }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.a
    }
    pub fn direction(&self) -> &Vec3 {
        &self.b
    }
    pub fn point_at_param(&self, t: f32) -> Vec3 {
        (&self.a + &(&self.b * t))
    }
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray(a={}, b={})", self.origin(), self.direction())
    }
}

#[cfg(test)]
mod ray_tests {
    #[test]
    fn test() {
        use super::*;

        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 1.0, 1.0);
        let r = Ray::new(&orig, &dir);
        assert_eq!(r.origin(), &orig);
        assert_eq!(r.direction(), &dir);
        assert_eq!(r.point_at_param(1.0), Vec3::new(1.0, 1.0, 1.0));
    }
}
