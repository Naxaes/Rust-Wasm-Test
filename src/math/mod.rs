
type Unit = f32;

pub mod matrix {
    use nalgebra::Vector3;
    use nalgebra::Matrix4;

    struct Matrix4x4 {
        pub m00: f32, pub m01: f32, pub m02: f32, pub m03: f32,
        pub m10: f32, pub m11: f32, pub m12: f32, pub m13: f32,
        pub m20: f32, pub m21: f32, pub m22: f32, pub m23: f32,
        pub m30: f32, pub m31: f32, pub m32: f32, pub m33: f32,
    }

    impl Matrix4x4 {
        pub fn identity() -> Self {
            Self {
                m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
                m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
                m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
                m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
            }
        }
    }

    // pub fn make_matrix(position: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Matrix4<f32> {
    //
    // }

    type Mat4x4 = [f32; 16];

    pub fn translation(tx: f32, ty: f32, tz: f32) -> Mat4x4 {
        let mut matrix = [0f32; 16];

        matrix[0] = 1.;
        matrix[5] = 1.;
        matrix[10] = 1.;
        matrix[15] = 1.;

        matrix[12] = tx;
        matrix[13] = ty;
        matrix[14] = tz;

        matrix
    }

    pub fn scale(sx: f32, sy: f32, sz: f32) -> Mat4x4 {
        let mut matrix = [0f32; 16];

        matrix[0] = sx;
        matrix[5] = sy;
        matrix[10] = sz;
        matrix[15] = 1.;

        matrix
    }

    // pub fn perspective(aspect_ratio: f32, fov_y: f32, far_plane: f32, near_plane: f32) -> Mat4x4 {
    //     let mut m = [0f32; 16];
    //
    //     m[0]  = (2*n) / (r - l);  m[4]  = 0.;               m[8]  =  (r + l) / (r - l);  m[12] = 0.;
    //     m[1]  = 0.;               m[5]  = (2*n) / (t - b);  m[9]  =  (t + b) / (t - b);  m[13] = 0.;
    //     m[2]  = 0.;               m[6]  = 0.;               m[10] = -(f + n) / (f - n);  m[14] = -(2*f*n) / (f - n);
    //     m[3]  = 0.;               m[7]  = 0.;               m[11] = -1.0;                m[15] = 0.;
    //
    //     m
    // }

    pub fn multiply_mat4x4(a: Mat4x4, b: Mat4x4) -> Mat4x4 {
        let mut matrix = [0f32; 16];

        matrix[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
        matrix[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
        matrix[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
        matrix[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

        matrix[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
        matrix[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
        matrix[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
        matrix[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

        matrix[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
        matrix[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
        matrix[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
        matrix[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

        matrix[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
        matrix[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
        matrix[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
        matrix[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

        matrix
    }
}