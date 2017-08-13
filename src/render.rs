use image::{self, ImageBuffer, RgbImage};
use palette::{self, Limited, Rgb};
use std::f32;

use camera::Camera;
use material::Reflection;
use ray::Ray;
use scene::Scene;

#[derive(Copy, Clone, Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub fov: u32,
    pub samples_per_pixel: u32,
    pub max_reflections: u32,
}

impl Renderer {
    pub fn render(&self, scene: &Scene) -> RgbImage {
        let camera = Camera::new(self.width, self.height, self.fov);

        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f32 - (self.width / 2) as f32;
            let y = j as f32 - (self.height / 2) as f32;
            let mut color = palette::Rgb::new(0.0, 0.0, 0.0);
            for _ in 0..self.samples_per_pixel {
                color = color + self.trace(scene, camera.get_ray(x, y), 0);
            }
            color = (color / self.samples_per_pixel as f32).clamp();
            let srgb = palette::pixel::Srgb::from(color);
            image::Rgb([to_u8(srgb.red), to_u8(srgb.green), to_u8(srgb.blue)])
        })
    }

    fn trace(&self, scene: &Scene, ray: Ray, reflections: u32) -> Rgb {
        if reflections > self.max_reflections {
            return scene.global_illumination;
        }

        match scene.sample(ray) {
            Some(sample) => {
                let reflected = match sample.reflection {
                    Some(Reflection { ray, intensity }) => {
                        self.trace(scene, ray, reflections + 1) * intensity
                    }
                    None => Rgb::new(0.0, 0.0, 0.0),
                };
                sample.color * (reflected + sample.emission)
            }
            _ => scene.global_illumination,
        }
    }
}

#[inline]
fn to_u8(value: f32) -> u8 {
    (value * 255.0) as u8
}
