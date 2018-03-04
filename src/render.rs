use image;
use palette::{self, Limited};
use rand::{self, Rng};
use rayon::prelude::*;
use std::f32;

use camera::Camera;
use material::Sample;
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
    pub fn render(&self, scene: &Scene) -> image::RgbImage {
        let camera = Camera::new(scene.camera_ray, self.width, self.fov);

        let pixels: Vec<Vec<_>> = (0..self.width)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::weak_rng();
                (0..self.height)
                    .into_iter()
                    .map(|j| {
                        let x = i as f32 - (self.width / 2) as f32;
                        let y = (self.height / 2) as f32 - j as f32;

                        let mut color = palette::LinSrgb::new(0.0, 0.0, 0.0);
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
                        palette::Srgb::linear_to_pixel(color)

                    })
                    .collect()
            })
            .collect();

        let mut image = image::ImageBuffer::new(self.width, self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                image.put_pixel(i, j, image::Rgb(pixels[i as usize][j as usize]));
            }
        }
        image
    }

    fn trace(&self,
             scene: &Scene,
             rng: &mut Rng,
             ray: Ray,
             reflections: u32)
             -> palette::LinSrgb {
        if reflections > self.max_reflections {
            return scene.global_illumination;
        }

        scene.sample(rng, ray).map_or(scene.global_illumination, |sample| {
            match sample {
                Sample::Emit(color) => color,
                Sample::Bounce(color, ray) => {
                    color * self.trace(scene, rng, ray, reflections + 1)
                }
            }
        })
    }
}
