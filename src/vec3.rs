use std::ops;

use crate::rtweekend::{random_double, random_double_range};

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

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn random() -> Self {
        Self(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
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

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let len_sq = p.length_squared();
            // avoiding vectors really close to the center, as by normalizing we'll get Vec(+-inf, +-inf, +-inf)
            // and vectors outside the sphere
            if 1e-160 < len_sq && len_sq <= 1.0 {
                return p / len_sq.sqrt(); // normalize vector
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Self::dot(v, n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, idxref_ratio: f64) -> Vec3 {
        let r_prime_perp = idxref_ratio * (*uv + Vec3::dot(&-*uv, n).min(1.0) * *n);
        let r_prime_paral = -f64::sqrt((1.0 - r_prime_perp.length_squared()).abs()) * *n;
        r_prime_perp + r_prime_paral
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

pub mod color {
    use crate::interval::*;

    use super::Vec3 as Color;

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        0.0
    }

    pub fn write_color(pixel_color: &Color) -> String {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        // Apply a linear to gamma transform for gamma 2
        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // Translate the <0, 1> component values values to the byte range <0, 255>
        let intensity = Interval::new(0.000, 0.999); // ensure values are in the correct range
        let rbyte = (256.0 * intensity.clamp(r)) as i32;
        let gbyte = (256.0 * intensity.clamp(g)) as i32;
        let bbyte = (256.0 * intensity.clamp(b)) as i32;

        format!("#{:02X}{:02X}{:02X}", rbyte, gbyte, bbyte)
    }
}
