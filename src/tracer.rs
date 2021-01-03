use rand::Rng;

use crate::bvh::BoundingVolumeHierarchy;
use crate::ray::Ray;
use crate::scene::Scene;

pub struct PathTracer<'a, R: Rng + ?Sized> {
    scene: &'a Scene,
    bvh: &'a BoundingVolumeHierarchy<'a>,
    rng: &'a mut R,
    max_reflections: u32,
    reflections: u32,
}

impl<'a, R: Rng + ?Sized> PathTracer<'a, R> {
    pub fn new(
        scene: &'a Scene,
        bvh: &'a BoundingVolumeHierarchy<'_>,
        rng: &'a mut R,
        max_reflections: u32,
    ) -> Self {
        PathTracer {
            scene,
            bvh,
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

        self.reflections += 1;
        self.bvh
            .sample(ray)
            .map(|sample| sample.material.sample(self, &sample.intersection))
            .unwrap_or(self.scene.global_illumination)
    }
}
