use image::{self, ImageBuffer, RgbImage};
use nalgebra::{Origin, Point3, Vector3};
use palette;
use std::f64;

use scene::Scene;
use ray::Ray;
use render::render_ray;

const OVERSAMPLE_FACTOR: isize = 3;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pos: Point3<f64>,
    z: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: u32) -> Self {
        assert!(0 < fov && fov < 180);
        Camera {
            width: width,
            height: height,
            pos: Point3::origin(),
            z: (width as f64 / 2.0) /
               (fov as f64 * f64::consts::PI / 180.0 / 2.0).tan(),
        }
    }

    pub fn draw(&self, scene: &Scene) -> RgbImage {
        let oversample_deltas: Vec<_> = (0..OVERSAMPLE_FACTOR)
            .map(|i| {
                (i - OVERSAMPLE_FACTOR / 2) as f64 / OVERSAMPLE_FACTOR as f64
            })
            .collect();
        let num_oversamples = OVERSAMPLE_FACTOR * OVERSAMPLE_FACTOR;
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f64 - (self.width / 2) as f64;
            let y = j as f64 - (self.height / 2) as f64;
            let mut color = palette::Rgb::new(0.0, 0.0, 0.0);
            for dx in &oversample_deltas {
                for dy in &oversample_deltas {
                    let ray = Ray::new(self.pos,
                                       Vector3::new(x + dx, y + dy, self.z));
                    color = color + render_ray(scene, ray);
                }
            }
            let pixel = palette::pixel::Srgb::from(color /
                                                   num_oversamples as f32);
            image::Rgb([to_u8(pixel.red),
                        to_u8(pixel.green),
                        to_u8(pixel.blue)])
        })
    }
}

#[inline]
fn to_u8(value: f32) -> u8 {
    (value * 255.0) as u8
}
