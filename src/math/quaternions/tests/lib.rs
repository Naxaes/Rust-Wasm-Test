#[cfg(test)]
mod tests {
    use quaternions::Quaternion;
    use nalgebra_glm::{Mat4, Vec3};

    pub fn print_matrix(m: &Mat4) {
        println!("\
Matrix(\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
)", m.m11, m.m12, m.m13, m.m14,
                 m.m21, m.m22, m.m23, m.m24,
                 m.m31, m.m32, m.m33, m.m34,
                 m.m41, m.m42, m.m43, m.m44
        );
    }

    fn default_rotation_matrix() -> Mat4 {
        let mut a = Mat4::default();
        a.fill_with_identity();
        a.m44 = 0.0;
        a
    }

    #[test]
    fn quaternion_to_matrix_default() {
        let quaternion = Quaternion::new();
        assert_eq!(quaternion.to_matrix(), default_rotation_matrix());
    }

    #[test]
    fn quaternion_to_matrix_rot_2pi_x() {
        let quaternion = Quaternion::from_axis_rotation(2.0 * std::f32::consts::PI, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(quaternion.to_matrix(), default_rotation_matrix());
    }
}