use image::{self, ImageBuffer, RgbImage};
use nalgebra::{Origin, Point3, Vector3};
use palette::{self, Limited};
use std::f32;

use scene::Scene;
use ray::Ray;

const OVERSAMPLE_FACTOR: isize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pos: Point3<f32>,
    z: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u32) -> Self {
        assert!(0 < fov && fov < 180);
        Camera {
            width: width,
            height: height,
            pos: Point3::origin(),
            z: (width as f32 / 2.0) /
               (fov as f32 * f32::consts::PI / 180.0 / 2.0).tan(),
        }
    }

    pub fn draw(&self, scene: &Scene) -> RgbImage {
        let oversample_deltas: Vec<_> = (0..OVERSAMPLE_FACTOR)
            .map(|i| {
                (i - OVERSAMPLE_FACTOR / 2) as f32 / OVERSAMPLE_FACTOR as f32
            })
            .collect();
        let num_oversamples = OVERSAMPLE_FACTOR * OVERSAMPLE_FACTOR;
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f32 - (self.width / 2) as f32;
            let y = j as f32 - (self.height / 2) as f32;
            let mut color = palette::Rgb::new(0.0, 0.0, 0.0);
            for dx in &oversample_deltas {
                for dy in &oversample_deltas {
                    let ray = Ray::new(self.pos,
                                       Vector3::new(x + dx, y + dy, self.z));
                    color = color + scene.trace(ray);
                }
            }
            color = (color / num_oversamples as f32).clamp();
            let srgb = palette::pixel::Srgb::from(color);
            image::Rgb([to_u8(srgb.red), to_u8(srgb.green), to_u8(srgb.blue)])
        })
    }
}

#[inline]
fn to_u8(value: f32) -> u8 {
    (value * 255.0) as u8
}
