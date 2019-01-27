use std::f32::consts::PI;

use crate::ray::Ray;
use crate::types::{Mat3, Point3, Vector3};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3,
    rotation: Mat3,
    z: f32,
}

impl Camera {
    pub fn new(camera_ray: Ray, width: u32, fov: u32) -> Self {
        assert!(0 < fov && fov < 180);
        Camera {
            origin: camera_ray.origin,
            rotation: make_rotation_matrix(&camera_ray.dir),
            z: (width as f32 / 2.0) / (fov as f32 * PI / 180.0 / 2.0).tan(),
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        // Create a view matrix pointing along the z axis, then rotate it to
        // face down the camera ray.
        Ray::new(self.origin, self.rotation * Vector3::new(x, y, self.z))
    }
}

fn make_rotation_matrix(axis: &Vector3) -> Mat3 {
    let mut pitch: f32 = axis.y.asin();
    let mut yaw: f32 = axis.x.asin();
    if axis.z > 0.0 {
        pitch *= -1.0;
    } else {
        yaw = std::f32::consts::PI - yaw;
    }

    let pitch_mat = Mat3::new([
        [1.0, 0.0, 0.0],
        [0.0, pitch.cos(), -pitch.sin()],
        [0.0, pitch.sin(), pitch.cos()],
    ]);
    let yaw_mat = Mat3::new([
        [yaw.cos(), 0.0, yaw.sin()],
        [0.0, 1.0, 0.0],
        [-yaw.sin(), 0.0, yaw.cos()],
    ]);
    pitch_mat * yaw_mat
}
