use nalgebra::{Vector3, partial_clamp};
use crate::log;
use wasm_bindgen::__rt::core::f32::consts::PI;
use std::cmp::{max, min};

type Millimeters = f32;
type Radians = f32;


pub struct PinholeCamera {
    pub focal_length: Millimeters,

    pub sensor_width:  Millimeters,
    pub sensor_height: Millimeters,

    pub aperture: Millimeters
}

impl PinholeCamera {
    pub fn new() -> Self {
        Self {
            focal_length: 8.0,
            sensor_width: 20.0,
            sensor_height: 20.0,
            aperture: 20.0,
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.sensor_width / self.sensor_height
    }

    pub fn fov_y(&self) -> f32 {
        let h = self.sensor_height;
        let f = self.focal_length;

        2.0 * (h / (2.0*f)).atan()
    }

    pub fn fov_x(&self) -> f32 {
        let w = self.sensor_width;
        let f = self.focal_length;

        2.0 * (w / (2.0*f)).atan()
    }
}


pub struct Axis {
    // Invariants:
    //   right   = normalize(right)
    //   up      = normalize(up)
    //   forward = normalize(forward)
    //   right * forwards = right * up = up * forward = 0  (scalar product)
    right:   Vector3<Millimeters>,
    up:      Vector3<Millimeters>,
    forward: Vector3<Millimeters>,

    yaw: Radians,
    pitch: Radians,
}

impl Axis {
    const WORLD_AXIS_RIGHT:   [f32; 3] = [1.0, 0.0, 0.0];
    const WORLD_AXIS_UP:      [f32; 3] = [0.0, 1.0, 0.0];
    const WORLD_AXIS_FORWARD: [f32; 3] = [0.0, 0.0, 1.0];

    pub fn new() -> Self {
        Self {
            right:   glm::make_vec3(&Axis::WORLD_AXIS_RIGHT),
            up:      glm::make_vec3(&Axis::WORLD_AXIS_UP),
            forward: glm::make_vec3(&Axis::WORLD_AXIS_FORWARD),

            yaw: PI / 2.0,
            pitch: PI / 2.0,
        }
    }

    fn clamp(value: f32, min: f32, max: f32) -> f32 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32) {
        let world_axis_up = glm::make_vec3(&Axis::WORLD_AXIS_UP);

        self.yaw += yaw;  // TODO(ted): fmod 2PI
        self.pitch = Self::clamp(self.pitch + pitch, -PI + 0.001, PI - 0.001);

        // The OpenGL-coordinate system is not like the traditional mathematical system, it's rotated.
        self.forward.x = self.yaw.cos() * self.pitch.sin();
        self.forward.y = self.pitch.cos();
        self.forward.z = self.yaw.sin() * self.pitch.sin();

        self.right = glm::cross(&world_axis_up, &self.forward);
        self.up    = glm::cross(&self.forward, &self.right);

        self.forward.normalize_mut();
        self.right.normalize_mut();
        self.up.normalize_mut();
    }
}


pub struct FPSCamera {
    pub direction: Axis,
    pub position: Vector3<Millimeters>,
    pub is_orthographic: bool,
    pub pinhole_camera: PinholeCamera,
}

impl FPSCamera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 3.0),
            direction: Axis::new(),
            is_orthographic: false,
            pinhole_camera: PinholeCamera::new(),
        }
    }

    /// Move the camera relative to it's direction ('move' is reserved, hence the name walk).
    pub fn move_right(&mut self, delta: f32) {
        self.position += &self.direction.right * delta;
    }
    pub fn move_up(&mut self, delta: f32) {
        self.position += &self.direction.up * delta;
    }
    pub fn move_forward(&mut self, delta: f32) {
        self.position += &self.direction.forward * delta;
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32, roll: f32) {
        self.direction.rotate(yaw, pitch, roll);
    }
}


pub type Camera = FPSCamera;