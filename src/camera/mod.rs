use nalgebra::Vector3;

type Millimeters = f32;



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



pub struct FPSCamera {
    pub position: Vector3<Millimeters>,
    pub direction: Vector3<Millimeters>,
    pub is_orthographic: bool,
    pub pinhole_camera: PinholeCamera,
}

impl FPSCamera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 3.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            is_orthographic: false,
            pinhole_camera: PinholeCamera::new(),
        }
    }
}


pub type Camera = FPSCamera;