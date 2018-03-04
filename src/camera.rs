use nalgebra::{Point3, Vector3};
use nalgebra::geometry::Rotation3;
use std::f32::consts::PI;

use ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Point3<f32>,
    rotation: Rotation3<f32>,
    z: f32,
}

impl Camera {
    pub fn new(camera_ray: Ray, width: u32, fov: u32) -> Self {
        assert!(0 < fov && fov < 180);
        Camera {
            origin: camera_ray.origin,
            rotation: Rotation3::new_observer_frame(
                &camera_ray.dir,
                &Vector3::y_axis(),
            ),
            z: (width as f32 / 2.0) / (fov as f32 * PI / 180.0 / 2.0).tan(),
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        // Create a view matrix pointing along the z axis, then rotate it to
        // face down the camera ray.
        Ray::new(self.origin, self.rotation * Vector3::new(x, y, self.z))
    }
}
