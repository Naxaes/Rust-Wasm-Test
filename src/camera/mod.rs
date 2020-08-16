use metric::length::metric::{Millimeters};
use nalgebra::Vector3;


pub struct Camera {
    position: Vector3<Millimeters>,

    focal_length: Millimeters,

    sensor_width:  Millimeters,
    sensor_height: Millimeters,

    aperture: Millimeters
}
