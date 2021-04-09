use std::ops::{Mul, Add, Sub};
use super::Real;


#[derive(Debug, PartialOrd, PartialEq, Copy, Clone, Default)]
pub struct Vector3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vector3  {
    pub const fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }

    pub const fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn norm(&self) -> Real {
        Real::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub const fn norm_squared(&self) -> Real {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub const fn dot(&self, other: &Self) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}


impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Real> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Real) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Real> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Real) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<Real> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Real) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}