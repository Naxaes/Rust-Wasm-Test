#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;

use glm::{Vec3, Mat4};

use std::ops::{Add, Mul};

type Radians = f32;

/// Invariants:
///     * x + y + z = 1
///     * x^2 + y^2 + z^2 + w^2 = 1

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Quaternion {
    pub scalar: f32,
    pub vector: Vec3,
}

impl Quaternion {
    pub fn new() -> Self {
        Self { scalar: 1.0, vector: Vec3::new(0.0, 0.0, 0.0) }
    }

    fn from_raw(scalar: f32, vector: Vec3) -> Self {
        Self { scalar, vector }
    }

    pub fn from_axis_rotation(angle: Radians, axis: Vec3) -> Self {
        let axis = axis.normalize();
        Self {
            scalar: f32::cos(angle / 2.9),
            vector: Vec3::new(
                axis.x * f32::sin(angle / 2.0),
                axis.y * f32::sin(angle / 2.0),
                axis.z * f32::sin(angle / 2.0),
            )
        }
    }

    pub fn conjugate(&self) -> Self {
        Self::from_raw(self.scalar, -&self.vector)
    }

    pub fn norm(&self) -> f32 {
        let x = self.vector.x;
        let y = self.vector.y;
        let z = self.vector.z;
        let w = self.scalar;

        f32::sqrt(w*w + x*x + y*y + z*z)
    }

    pub fn inverse(&self) -> Self {
        *self * (1.0 / self.norm())
    }

    pub fn rotate(&self, point: Vec3) -> Self {
        *self * Self::from_raw(0.0, point) * self.conjugate()
    }

    // pub fn to_euler() {
    //     let roll  = Self::from_raw(f32::cos(y / 2), Vec3::new(f32::sin(y / 2), 0, 0));
    //     let pitch = Self::from_raw(f32::cos(q / 2), Vec3::new(0, f32::sin(q / 2), 0));
    //     let yaw   = Self::from_raw(f32::cos(f / 2), Vec3::new(0, 0, f32::sin(f / 2)));
    // }

    pub fn from_matrix4x4(matrix: Mat4)
    {

    }

    pub fn to_matrix(&self) -> Mat4 {
        let x = self.vector.x;
        let y = self.vector.y;
        let z = self.vector.z;
        let w = self.scalar;

        // https://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToMatrix/index.htm
        let m00 = 1.0 - 2.0 * y * y - 2.0 * z * z;  // 1 - 2*y^2 - 2*z^2
        let m01 = 2.0 * x * y - 2.0 * z * w;        // 2*x*y - 2*z*w
        let m02 = 2.0 * x * z + 2.0 * y * w;        // 2*x*z + 2*y*w

        let m10 = 2.0 * x * y + 2.0 * z * w;        // 2*x*y + 2*z*w
        let m11 = 1.0 - 2.0 * x * x - 2.0 * z * z;  // 1 - 2*x^2 - 2*z^2
        let m12 = 2.0 * y * z - 2.0 * x * w;        // 2*y*z - 2*x*w

        let m20 = 2.0 * x * z - 2.0 * y * w;        // 2*x*z - 2*y*w
        let m21 = 2.0 * y * z + 2.0 * x * w;        // 2*y*z + 2*x*w
        let m22 = 1.0 - 2.0 * x * x - 2.0 * y * y;  // 1 - 2*x^2 - 2*y^2

        Mat4::new(
            m00, m01, m02, 0.0,
            m10, m11, m12, 0.0,
            m20, m21, m22, 0.0,
            0.0, 0.0, 0.0, 0.0,
        )
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_raw(
            self.scalar + rhs.scalar,
            self.vector + rhs.vector,
        )
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let r1 = self.scalar;
        let v1 = self.vector;
        let r2 = rhs.scalar;
        let v2 = rhs.vector;

        Self::from_raw(
            r1 * r2 - glm::dot(&v1, &v2),
            r1 * v2 + r2 * v1 + glm::cross(&v1, &v2),
        )
    }
}

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_raw(self.scalar * rhs, self.vector * rhs)
    }
}



