use palette::LinSrgb;
use rand::Rng;
use std::f32::consts::PI;

use crate::ray::Ray;
use crate::surface::Intersection;
use crate::tracer::PathTracer;
use crate::types::Vector3;

#[derive(Copy, Clone, Debug)]
enum Kind {
    Emissive,
    Diffuse,
    Specular,
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    color: LinSrgb,
    kind: Kind,
}

impl Material {
    pub fn light(color: LinSrgb) -> Self {
        Material {
            color,
            kind: Kind::Emissive,
        }
    }

    pub fn diffuse(color: LinSrgb) -> Self {
        Material {
            color,
            kind: Kind::Diffuse,
        }
    }

    pub fn specular(color: LinSrgb) -> Self {
        Material {
            color,
            kind: Kind::Specular,
        }
    }

    pub fn sample<R: Rng + ?Sized>(
        &self,
        tracer: &mut PathTracer<R>,
        int: &Intersection,
    ) -> LinSrgb {
        match self.kind {
            Kind::Emissive => self.color,
            Kind::Diffuse => {
                // Compuate Lambert BRDF with cosine sampling: this means the
                // contribution of each ray is directly proportional to its
                // probability.
                let dir = sample_hemisphere(tracer.rng(), int.normal, 1.0);
                self.color * tracer.trace(Ray::new(int.position, dir))
            }
            Kind::Specular => {
                let dir = int.incident
                    - 2.0 * int.normal.dot(int.incident) * int.normal;
                self.color * tracer.trace(Ray::new(int.position, dir))
            }
        }
    }
}

fn sample_hemisphere<R: Rng + ?Sized>(
    rng: &mut R,
    normal: Vector3,
    alpha: f32,
) -> Vector3 {
    // Sample a hemisphere, then project about the normal vector.
    let z = rng.gen::<f32>().powf(1.0 / (alpha + 1.0));
    let zp = (1.0 - z * z).sqrt();
    let theta = rng.gen::<f32>() * 2.0 * PI;
    normal.tangent_space() * Vector3::new(zp * theta.cos(), zp * theta.sin(), z)
}
