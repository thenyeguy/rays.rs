use palette::{self, Clamp, Pixel};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::f32;

use crate::profile;
use crate::scene::Scene;
use crate::tracer::PathTracer;

#[derive(Clone, Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_reflections: u32,
}

impl Renderer {
    pub fn render<F>(&self, scene: &Scene, on_col_done: F) -> image::RgbImage
    where
        F: Fn() + Sync,
    {
        profile::start("render.prof");
        let pixels: Vec<Vec<_>> = (0..self.width)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                let col = (0..self.height)
                    .into_iter()
                    .map(|j| {
                        let x = i as f32 - (self.width / 2) as f32;
                        let y = (self.height / 2) as f32 - j as f32;

                        let mut color = palette::LinSrgb::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let dx = rng.gen::<f32>() - 0.5;
                            let dy = rng.gen::<f32>() - 0.5;
                            let xnorm = (x + dx) / self.width as f32;
                            let ynorm = (y + dy) / self.width as f32;
                            let ray = scene.camera.get_ray(xnorm, ynorm);

                            let mut tracer = PathTracer::new(
                                scene,
                                &mut rng,
                                self.max_reflections,
                            );
                            color += tracer.trace(ray);
                        }
                        color = (color / self.samples_per_pixel as f32).clamp();
                        palette::Srgb::from_linear(color)
                            .into_format()
                            .into_raw()
                    })
                    .collect();
                on_col_done();
                col
            })
            .collect();
        profile::end();

        let mut image = image::ImageBuffer::new(self.width, self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                image.put_pixel(
                    i,
                    j,
                    image::Rgb(pixels[i as usize][j as usize]),
                );
            }
        }
        image
    }
}
