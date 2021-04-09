use glm::Mat4;
use super::log;


pub fn print_matrix(m: &Mat4) {
    log(format!("\
Matrix(\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
\t{:.2}, {:.2}, {:.2}, {:.2},\n\
)", m.m11, m.m12, m.m13, m.m14,
                m.m21, m.m22, m.m23, m.m24,
                m.m31, m.m32, m.m33, m.m34,
                m.m41, m.m42, m.m43, m.m44
    ).as_str());
}



pub fn create_grid(width: usize, depth: usize) -> (Vec<f32>, Vec<u16>) {
    let half_width = (width / 2) as i32;
    let half_depth = (depth / 2) as i32;

    let mut vertices = Vec::with_capacity(width * depth * 3);
    for x in -half_width..half_width {
        for z in -half_depth..half_depth {
            vertices.push(x as f32 + 0.5);
            vertices.push(0.0);
            vertices.push(z as f32 + 0.5);
        }
    }

    let mut indices = Vec::with_capacity(width + depth);
    for z in 0..depth {
        for x in 0..width {
            let index = z + x * depth;

            if z < (depth - 1) {
                indices.push(index as u16);
                indices.push((index + 1) as u16);
            }
            if x < (width - 1) {
                indices.push(index as u16);
                indices.push((index + depth) as u16);
            }
        }
    }

    (vertices, indices)

    // let grid = Mesh::from_f32_array_with_indices_3d(gl, &vertices, &indices).unwrap();
}


use wasm_bindgen::JsCast;
macro_rules! array_to_wasm_array {
    ($T:ident, $JSArray:ident, $array:ident) => {
        {
            let memory = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let location: u32 = ($array.as_ptr() as u32) / (std::mem::size_of::<$T>() as u32);  // TODO(ted): How does this work?

            $JSArray::new(&memory).subarray(
                location,
                location + $array.len() as u32,
            )
        }
    };
}


//
// /// The T type and JSArray type must match in size. I.e. ([f32] and Float32Array) or ([u16] and Uint16Array).
// pub fn array_to_wasm_array<T, JSArray>(array: &[T]) -> JSArray {
//     let memory = wasm_bindgen::memory()
//         .dyn_into::<WebAssembly::Memory>()
//         .unwrap()
//         .buffer();
//     let location: u32 = (array.as_ptr() as u32) / (std::mem::size_of::<T>() as u32);  // TODO(ted): How does this work?
//     let js_array = JSArray::new(&memory).subarray(
//         location,
//         location + array.len() as u32,
//     );
//
//     js_array
// }