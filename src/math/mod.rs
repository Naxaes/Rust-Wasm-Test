pub mod matrix {
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