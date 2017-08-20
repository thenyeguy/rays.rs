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

        let pixels: Vec<Vec<_>> = (0..self.width)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::weak_rng();
                (0..self.height)
                    .into_iter()
                    .map(|j| {
                        let x = i as f32 - (self.width / 2) as f32;
                        let y = (self.height / 2) as f32 - j as f32;

                        let mut color = palette::Rgb::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let dx = rng.next_f32() - 0.5;
                            let dy = rng.next_f32() - 0.5;
                            color = color +
                                    self.trace(scene,
                                               &mut rng,
                                               camera.get_ray(x + dx, y + dy),
                                               0);
                        }
                        color = (color / self.samples_per_pixel as f32).clamp();

                        let srgb = palette::pixel::Srgb::from(color);
                        image::Rgba(srgb.to_pixel())
                    })
                    .collect()
            })
            .collect();

        let mut image = ImageBuffer::new(self.width, self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                image.put_pixel(i, j, pixels[i as usize][j as usize]);
            }
        }
        image
    }

    fn trace(&self,
             scene: &Scene,
             rng: &mut Rng,
             ray: Ray,
             reflections: u32)
             -> Rgb {
        if reflections > self.max_reflections {
            return scene.global_illumination;
        }

        match scene.sample(rng, ray) {
            Some(sample) => {
                let reflected = match sample.reflection {
                    Some(Reflection { ray, intensity }) => {
                        self.trace(scene, rng, ray, reflections + 1) * intensity
                    }
                    None => Rgb::new(0.0, 0.0, 0.0),
                };
                sample.color * (reflected + sample.emission)
            }
            _ => scene.global_illumination,
        }
    }
}
