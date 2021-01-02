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
                // Generate a random direction vector.
                let theta = tracer.rng().gen::<f32>() * 2.0 * PI;
                let z = tracer.rng().gen::<f32>();
                let zp = (1.0 - z * z).sqrt();
                let mut dir =
                    Vector3::new(zp * theta.cos(), zp * theta.sin(), z);

                // Ensure we sample only from a hemisphere
                let mut intensity = dir.dot(int.normal);
                if intensity < 0.0 {
                    dir = dir * -1.0;
                    intensity *= -1.0;
                }

                // Recurse and combine colors.
                self.color
                    * intensity
                    * tracer.trace(Ray::new(int.position, dir))
            }
            Kind::Specular => {
                let dir = int.incident
                    - 2.0 * int.normal.dot(int.incident) * int.normal;
                self.color * tracer.trace(Ray::new(int.position, dir))
            }
        }
    }
}
