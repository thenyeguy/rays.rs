use image::{self, ImageBuffer, RgbaImage};
use palette::{self, Limited, Rgb};
use rand::{self, Rng};
use rayon::prelude::*;
use std::f32;

use camera::Camera;
use material::Reflection;
use ray::Ray;
use scene::Scene;

#[derive(Clone, Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub fov: u32,
    pub samples_per_pixel: u32,
    pub max_reflections: u32,
}

impl Renderer {
    pub fn render(&self, scene: &Scene) -> RgbaImage {
        let camera = Camera::new(self.width, self.height, self.fov);

        let pixels: Vec<_> = (0..self.width * self.height)
            .into_par_iter()
            .map(|n| {
                let x = (n % self.width) as f32 - (self.width / 2) as f32;
                let y = (self.height / 2) as f32 - (n / self.width) as f32;

                let mut color = palette::Rgb::new(0.0, 0.0, 0.0);
                let mut rng = rand::thread_rng();
                for _ in 0..self.samples_per_pixel {
                    let dx = rng.next_f32() - 0.5;
                    let dy = rng.next_f32() - 0.5;
                    color =
                        color +
                        self.trace(scene, camera.get_ray(x + dx, y + dy), 0);
                }
                color = (color / self.samples_per_pixel as f32).clamp();

                let srgb = palette::pixel::Srgb::from(color);
                image::Rgba(srgb.to_pixel())
            })
            .collect();

        let mut image = ImageBuffer::new(self.width, self.height);
        for n in 0..self.width * self.height {
            image.put_pixel(n % self.width, n / self.width, pixels[n as usize]);
        }
        image
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
