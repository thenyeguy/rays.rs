use rand::Rng;

use crate::bounds::BoundingVolumeHierarchy;
use crate::material::Sample;
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
        max_reflections: u32
    ) -> Self {
        PathTracer {
            scene,
            bvh,
            rng,
            max_reflections,
            reflections: 0,
        }
    }

    pub fn trace(&mut self, ray: Ray) -> palette::LinSrgb {
        if self.reflections > self.max_reflections {
            return self.scene.global_illumination;
        }

        self.reflections += 1;
        self.bvh.sample(self.rng, ray)
            .map_or(self.scene.global_illumination, |sample| match sample {
                Sample::Emit(color) => color,
                Sample::Bounce(color, ray) => {
                    color * self.trace(ray)
                }
            })
    }
}
