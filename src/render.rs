use palette::{self, Limited, Pixel};
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::f32;

use crate::bvh::BoundingVolumeHierarchy;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::tracer::PathTracer;

pub trait RenderProgress: Sync {
    fn on_render_start(&self);
    fn on_row_done(&self);
    fn on_render_done(&self);
}

#[derive(Clone, Debug)]
pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub fov: u32,
    pub samples_per_pixel: u32,
    pub max_reflections: u32,
}

impl Renderer {
    pub fn render(
        &self,
        scene: &Scene,
        progress: &dyn RenderProgress,
    ) -> image::RgbImage {
        let bvh = BoundingVolumeHierarchy::new(scene);
        let camera = Camera::new(scene.camera_ray, self.width, self.fov);
        progress.on_render_start();
        let pixels: Vec<Vec<_>> = (0..self.width)
            .into_par_iter()
            .map(|i| {
                let mut rng = rand::rngs::SmallRng::from_entropy();
                let row = (0..self.height)
                    .into_iter()
                    .map(|j| {
                        let x = i as f32 - (self.width / 2) as f32;
                        let y = (self.height / 2) as f32 - j as f32;

                        let mut color = palette::LinSrgb::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let dx = rng.gen::<f32>() - 0.5;
                            let dy = rng.gen::<f32>() - 0.5;
                            let mut tracer = PathTracer::new(
                                &scene,
                                &bvh,
                                &mut rng,
                                self.max_reflections,
                            );
                            color = color
                                + tracer.trace(camera.get_ray(x + dx, y + dy));
                        }
                        color = (color / self.samples_per_pixel as f32).clamp();
                        palette::Srgb::from_linear(color)
                            .into_format()
                            .into_raw()
                    })
                    .collect();
                progress.on_row_done();
                row
            })
            .collect();
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
