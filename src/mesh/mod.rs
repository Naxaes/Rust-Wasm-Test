use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
use web_sys::*;
use web_sys::WebGl2RenderingContext as GL;
use crate::log;


pub const VERTICES_2D_RECTANGLE: [f32; 12] = [
    -1.,  1., // x, y
    -1., -1., // x, y
     1.,  1., // x, y
     1.,  1., // x, y
    -1., -1., // x, y
     1., -1., // x, y
];
pub const VERTICES_2D_TRIANGLE: [f32; 6] = [
    -1., 1., // x, y
    -1., -1., // x, y
     1., 1., // x, y
];

pub const VERTICES_2D_INDEXED_RECTANGLE: [f32; 8] = [
    -1.,  1., // x, y
    -1., -1., // x, y
     1.,  1., // x, y
     1., -1., // x, y
];
pub const VERTICES_3D_INDEXED_RECTANGLE: [f32; 12] = [
    -1.,  1.,  0.,
    -1., -1.,  0.,
     1.,  1.,  0.,
     1., -1.,  0.,
];
pub const INDICES_RECTANGLE: [u16; 6] = [
    0, 1, 2,
    2, 1, 3
];
pub const VERTICES_TEXTURE_AND_NORMAL_3D_CUBE: [f32; 288] = [
//    Positions          Texture coordinates       Normals
     0.5, -0.5, -0.5,       1.0,  0.0,        0.0,  0.0, -1.0,
    -0.5, -0.5, -0.5,       0.0,  0.0,        0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,       1.0,  1.0,        0.0,  0.0, -1.0,
    -0.5,  0.5, -0.5,       0.0,  1.0,        0.0,  0.0, -1.0,
     0.5,  0.5, -0.5,       1.0,  1.0,        0.0,  0.0, -1.0,
    -0.5, -0.5, -0.5,       0.0,  0.0,        0.0,  0.0, -1.0,

    -0.5, -0.5,  0.5,       0.0,  0.0,        0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,       1.0,  1.0,        0.0,  0.0,  1.0,
     0.5, -0.5,  0.5,       1.0,  0.0,        0.0,  0.0,  1.0,
    -0.5,  0.5,  0.5,       0.0,  1.0,        0.0,  0.0,  1.0,
     0.5,  0.5,  0.5,       1.0,  1.0,        0.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,       0.0,  0.0,        0.0,  0.0,  1.0,

    -0.5,  0.5,  0.5,       1.0,  0.0,        -1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,       1.0,  1.0,        -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,       0.0,  1.0,        -1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,       0.0,  1.0,        -1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5,       0.0,  0.0,        -1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5,       1.0,  0.0,        -1.0,  0.0,  0.0,

     0.5,  0.5,  0.5,       1.0,  0.0,        1.0,  0.0,  0.0,
     0.5,  0.5, -0.5,       1.0,  1.0,        1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,       0.0,  1.0,        1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,       0.0,  1.0,        1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,       0.0,  0.0,        1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,       1.0,  0.0,        1.0,  0.0,  0.0,

    -0.5, -0.5, -0.5,       0.0,  1.0,        0.0, -1.0,  0.0,
     0.5, -0.5, -0.5,       1.0,  1.0,        0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,       1.0,  0.0,        0.0, -1.0,  0.0,
     0.5, -0.5,  0.5,       1.0,  0.0,        0.0, -1.0,  0.0,
    -0.5, -0.5,  0.5,       0.0,  0.0,        0.0, -1.0,  0.0,
    -0.5, -0.5, -0.5,       0.0,  1.0,        0.0, -1.0,  0.0,

    -0.5,  0.5, -0.5,       0.0,  1.0,        0.0,  1.0,  0.0,
     0.5,  0.5, -0.5,       1.0,  1.0,        0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,       1.0,  0.0,        0.0,  1.0,  0.0,
     0.5,  0.5,  0.5,       1.0,  0.0,        0.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,       0.0,  0.0,        0.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,       0.0,  1.0,        0.0,  1.0,  0.0
];



pub struct Mesh {
    id: WebGlVertexArrayObject,
    count: i32,
    is_indexed: bool,
}

impl Mesh {
    const DIMENSIONS: usize = 3;

    pub fn enable(&self, gl: &GL) {
        gl.bind_vertex_array(Some(&self.id));
    }

    pub fn draw(&self, gl: &GL) {
        if self.is_indexed {
            gl.draw_elements_with_i32(GL::TRIANGLES, self.count, GL::UNSIGNED_SHORT, 0);
        } else {
            gl.draw_arrays(GL::TRIANGLES, 0, self.count);
        }
    }

    pub fn from_f32_array(gl: &GL, vertices: &[f32]) -> Result<Self, String> {
        assert_eq!(vertices.len() % Self::DIMENSIONS, 0);

        let vertex_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location: u32 = vertices.as_ptr() as u32 / 4;  // TODO(ted): How does this work?
        let vertices_array = js_sys::Float32Array::new(&vertex_memory).subarray(
            vertices_location,
            vertices_location + vertices.len() as u32,
        );

        // Create vertex array buffer to store vertex buffers and element buffers.
        let vao = gl.create_vertex_array().ok_or("[WEBGL2 - VAO ERROR]: Unable to create VAO.")?;
        gl.bind_vertex_array(Some(&vao));

        // Create vertex buffer to put our data into video memory.
        let vbo = gl.create_buffer().ok_or("[WEBGL2 - VAO ERROR]: Unable to create VBO.")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        // Tell OpenGL the data's format.
        gl.enable_vertex_attrib_array(0);
        gl.enable_vertex_attrib_array(1);
        gl.enable_vertex_attrib_array(2);

        let stride = (3 + 2 + 3) * (4 /* sizeof(float) */);
        gl.vertex_attrib_pointer_with_i32(0, 3 as i32, GL::FLOAT, false, stride, 0);
        gl.vertex_attrib_pointer_with_i32(1, 2 as i32, GL::FLOAT, false, stride, 3 * (4 /* sizeof(float) */));
        gl.vertex_attrib_pointer_with_i32(2, 3 as i32, GL::FLOAT, false, stride, (3 + 2) * (4 /* sizeof(float) */));

        Ok(Self { id: vao, count: (vertices.len() / (3 + 2 + 3)) as i32, is_indexed: false})
    }

    pub fn from_f32_array_with_indices(gl: &GL, vertices: &[f32], indices: &[u16]) -> Result<Self, String> {
        assert_eq!(indices.len() % Self::DIMENSIONS, 0);

        let vertex_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location: u32 = vertices.as_ptr() as u32 / 4;  // TODO(ted): How does this work?
        let vertices_array = js_sys::Float32Array::new(&vertex_memory).subarray(
            vertices_location,
            vertices_location + vertices.len() as u32,
        );

        let index_memory = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let indices_location: u32 = indices.as_ptr() as u32 / 2;  // TODO(ted): How does this work?
        let indices_array = js_sys::Uint16Array::new(&index_memory).subarray(
            indices_location,
            indices_location + indices.len() as u32,
        );


        // Create vertex array buffer to store vertex buffers and element buffers.
        let vao = gl.create_vertex_array().ok_or("[WEBGL2 - VAO ERROR]: Unable to create VAO.")?;
        gl.bind_vertex_array(Some(&vao));

        // Create an element buffer to put our data into video memory.
        let ebo = gl.create_buffer().ok_or("[WEBGL2 - VAO ERROR]: Unable to create EBO.")?;
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

        // Create vertex buffer to put our data into video memory.
        let vbo = gl.create_buffer().ok_or("[WEBGL2 - VAO ERROR]: Unable to create VBO.")?;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        // Tell OpenGL the data's format.
        gl.enable_vertex_attrib_array(0);

        gl.vertex_attrib_pointer_with_i32(0, Self::DIMENSIONS as i32, GL::FLOAT, false, 0, 0);

        Ok(Self { id: vao, count: indices.len() as i32, is_indexed: true})
    }
}