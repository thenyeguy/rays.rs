use palette::{self, Limited, Pixel};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::f32;

use crate::profile;
use crate::scene::Scene;
use crate::tracer::PathTracer;

pub trait RenderProgress: Sync {
    fn on_render_start(&self);
    fn on_col_done(&self);
    fn on_render_done(&self);
}

#[derive(Clone, Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_reflections: u32,
}

impl Renderer {
    pub fn render(
        &self,
        scene: &Scene,
        progress: &dyn RenderProgress,
    ) -> image::RgbImage {
        profile::start("render.prof");
        progress.on_render_start();
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
                                &scene,
                                &mut rng,
                                self.max_reflections,
                            );
                            color = color + tracer.trace(ray);
                        }
                        color = (color / self.samples_per_pixel as f32).clamp();
                        palette::Srgb::from_linear(color)
                            .into_format()
                            .into_raw()
                    })
                    .collect();
                progress.on_col_done();
                col
            })
            .collect();
        profile::end();
        progress.on_render_done();

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
