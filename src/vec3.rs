#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }
    pub fn length(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
    }
    pub fn squared_length(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        let sum = [
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ];
        Vec3 { e: sum }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        let sum = [
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ];
        Vec3 { e: sum }
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        let diff = [
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ];
        Vec3 { e: diff }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        let diff = [
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ];
        Vec3 { e: diff }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Self::Output {
        Vec3::new(self.x() * s, self.y() * s, self.z() * s)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Self::Output {
        Vec3::new(self.x() * s, self.y() * s, self.z() * s)
    }
}

impl std::ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, divisor: f32) -> Self::Output {
        Vec3::new(self.x() / divisor, self.y() / divisor, self.z() / divisor)
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x(), self.y(), self.z())
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    (v1.e[0] * v2.e[0]) + (v1.e[1] * v2.e[1]) + (v1.e[2] * v2.e[2])
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
    )
}

pub fn is_close(v1: &Vec3, v2: &Vec3, thresh: f32) -> bool {
    let diff = v1 - v2;
    diff.e[0] < thresh && diff.e[1] < thresh && diff.e[2] < thresh
}

pub fn red() -> Vec3 {
    Vec3::new(1.0, 0.0, 0.0)
}

#[cfg(test)]
mod vec_tests {
    #[test]
    fn test() {
        use super::*;

        let v = Vec3::new(4.0, 8.0, 19.0);
        let mut x = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(v.length(), 21.0);
        assert_eq!(-v, Vec3::new(-4.0, -8.0, -19.0));
        assert_eq!(dot(&x, &v), 62.0);
        assert_eq!(&v * 2.0, Vec3::new(8.0, 16.0, 38.0));
        assert!(is_close(
            &unit_vector(&Vec3::new(1.0, 1.0, 1.0)),
            &Vec3::new(0.577, 0.577, 0.577),
            0.001
        ));
        assert!(is_close(
            &unit_vector(&Vec3::new(1.0, 0.0, 0.0)),
            &Vec3::new(1.0, 0.0, 0.0),
            0.001
        ));

        x.make_unit_vector();
        assert!(is_close(&x, &Vec3::new(0.577, 0.577, 0.577), 0.001));

        assert_eq!(
            cross(&Vec3::new(1.0, 1.0, 1.0), &Vec3::new(1.0, 1.0, 1.0)),
            Vec3::new(0.0, 0.0, 0.0)
        );
        assert_eq!(
            cross(&Vec3::new(-1.0, 1.0, 1.0), &Vec3::new(1.0, 1.0, 1.0)),
            Vec3::new(0.0, 2.0, -2.0)
        );
    }
}
