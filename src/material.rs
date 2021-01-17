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
    Reflective {
        index: f32,
        roughness: f32,
        metallic: bool,
    },
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
        Material::glossy(color, 1.0, 1.0)
    }

    pub fn specular(color: LinSrgb) -> Self {
        Material::metallic(color, 1.5, 0.0)
    }

    pub fn glossy(color: LinSrgb, index: f32, roughness: f32) -> Self {
        let metallic = false;
        Material {
            color,
            kind: Kind::Reflective {
                index,
                roughness,
                metallic,
            },
        }
    }

    pub fn metallic(color: LinSrgb, index: f32, roughness: f32) -> Self {
        let metallic = true;
        Material {
            color,
            kind: Kind::Reflective {
                index,
                roughness,
                metallic,
            },
        }
    }

    pub fn sample<R: Rng + ?Sized>(
        &self,
        tracer: &mut PathTracer<R>,
        int: &Intersection,
    ) -> LinSrgb {
        match self.kind {
            Kind::Emissive => self.color,
            Kind::Reflective {
                index,
                roughness,
                metallic,
            } => {
                // Estimate the Fresnel coefficient of the surface normal, and
                // use it to weight the specularity.
                let f0 = if metallic {
                    (self.color.red + self.color.green + self.color.blue) / 3.0
                } else {
                    ((1.0 - index) / (1.0 + index)).powi(2)
                };
                let dot = int.normal.dot(int.incident).abs();
                let fresnel = f0 + (1.0 - f0) * (1.0 - dot).powi(5);
                if tracer.rng().gen::<f32>() < fresnel {
                    // Specular: Phong-Blinn BRDF.
                    let reflected = int.incident
                        - 2.0 * int.normal.dot(int.incident) * int.normal;
                    let dir =
                        sample_hemisphere(tracer.rng(), reflected, roughness);
                    tracer.trace(Ray::new(int.position, dir))
                } else if !metallic {
                    // Diffuse: Lambert BRDF with cosine sampling.
                    let dir = sample_hemisphere(tracer.rng(), int.normal, 1.0);
                    self.color * tracer.trace(Ray::new(int.position, dir))
                } else {
                    LinSrgb::new(0.0, 0.0, 0.0)
                }
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
