use nalgebra::{self as na, Point3, Vector3};
use std::f32::consts::PI;

use ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    width: u32,
    height: u32,
    pos: Point3<f32>,
    z: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u32) -> Self {
        assert!(0 < fov && fov < 180);
        Camera {
            width: width,
            height: height,
            pos: na::origin(),
            z: (width as f32 / 2.0) /
                (fov as f32 * PI / 180.0 / 2.0).tan(),
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(self.pos, Vector3::new(x, y, self.z))
    }
}
