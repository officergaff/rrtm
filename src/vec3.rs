use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::utils::{random_double, random_double_range};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    // NEED TO REVISIT -----
    pub fn random() -> Self {
        Self {
            e: [random_double(), random_double(), random_double()],
        }
    }
    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            e: [
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
    }
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            let lensq = p.length_squared();
            if lensq > 1e-160 && lensq <= 1. {
                // First conditional to avoid floating point
                // abstraction leak
                return p / f64::sqrt(lensq);
            }
        }
    }
    pub fn random_on_hemisphere(normal: &Self) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if dot(on_unit_sphere, *normal) > 0. {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }
    pub fn reflect(v: &Self, n: &Self) -> Self {
        return *v - *n * (2. * dot(*v, *n));
    }
    pub fn refract(uv: &Self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = f64::min(dot(-*uv, *n), 1.0);
        let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
        let r_out_parallel = *n * -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared()));

        // Ensure the resulting vector is normalized
        unit_vector(&(r_out_perp + r_out_parallel))
    }
    // NEED TO REVISIT -----

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e.iter().fold(0., |acc, num| acc + num * num)
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to 0 in all dimensions
        let s = 1e-8;
        return f64::abs(self.e[0]) < s && f64::abs(self.e[1]) < s && f64::abs(self.e[2]) < s;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    return *v / v.length();
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self {
            e: [self[0] / t, self[1] / t, self[2] / t],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {
            e: [self[0] * t, self[1] * t, self[2] * t],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let v = Vec3::new(2., 1., 1.);
        assert_eq!(v.length(), f64::sqrt(6.0));
    }
    #[test]
    fn length_squared() {
        let v = Vec3::new(2., 1., 1.);
        assert_eq!(v.length_squared(), 6.0);
    }
}
