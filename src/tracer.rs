use rand::Rng;

use crate::ray::Ray;
use crate::scene::Scene;
use crate::{increment_statistic, statistics};

pub struct PathTracer<'a, R: Rng + ?Sized> {
    scene: &'a Scene,
    rng: &'a mut R,
    max_reflections: u32,
    reflections: u32,
}

impl<'a, R: Rng + ?Sized> PathTracer<'a, R> {
    pub fn new(scene: &'a Scene, rng: &'a mut R, max_reflections: u32) -> Self {
        PathTracer {
            scene,
            rng,
            max_reflections,
            reflections: 0,
        }
    }

    pub fn rng(&mut self) -> &mut R {
        self.rng
    }

    pub fn trace(&mut self, ray: Ray) -> palette::LinSrgb {
        if self.reflections > self.max_reflections {
            return self.scene.global_illumination;
        }

        increment_statistic!(statistics::RAYS_CAST);
        self.reflections += 1;
        self.scene
            .objects
            .sample(ray)
            .map(|sample| sample.material.sample(self, &sample.intersection))
            .unwrap_or(self.scene.global_illumination)
    }
}
