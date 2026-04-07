use std::ops;
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(v: &Self) -> Self {
        *v / v.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

// Vec3 utility functions
// impl ops::Shl
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self * other.0, self * other.1, self * other.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    // fn mul(self, t: f64) -> Self {
    //     Self(t * self.0, t * self.1, t * self.2)
    // }
    fn mul(self, t: f64) -> Self {
        t * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        (1.0 / t) * self
    }
}

pub enum Vec3Enum {
    Point3(Vec3),
    Color(Vec3),
}

pub mod color {
    use super::Vec3 as Color;

    pub fn write_color(pixel_color: &Color) -> String {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let rbyte: u16 = (255.0 * r) as u16;
        let gbyte: u16 = (255.0 * g) as u16;
        let bbyte: u16 = (255.0 * b) as u16;

        // println!("{rbyte} {gbyte} {bbyte}");
        format!("#{:02X}{:02X}{:02X}", rbyte, gbyte, bbyte)
    }
}
