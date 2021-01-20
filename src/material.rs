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
                let n_dot_i = int.normal.dot(int.incident).abs();
                let fresnel = f0 + (1.0 - f0) * (1.0 - n_dot_i).powi(5);

                if tracer.rng().gen::<f32>() < fresnel {
                    // Sample a GGX microfacet, and use it to compute a
                    // reflected ray.
                    let microfacet =
                        sample_ggx(tracer.rng(), int.normal, roughness);
                    let outgoing = int.incident
                        - 2.0 * microfacet.dot(int.incident) * microfacet;

                    // Validate this ray is visible.
                    let m_dot_o = microfacet.dot(outgoing);
                    let n_dot_o = int.normal.dot(outgoing);
                    if m_dot_o < 0.0 || n_dot_o < 0.0 {
                        return LinSrgb::default();
                    }

                    // GGX distribution function (after importance sampling):
                    let m_dot_i = microfacet.dot(int.incident).abs();
                    let m_dot_n = microfacet.dot(int.normal);
                    let distribution = m_dot_i / (2.0 * n_dot_i * m_dot_n);

                    // Smith correlated shadow masking function:
                    let r2 = roughness.powi(2);
                    let left =
                        n_dot_i * (r2 + (1.0 - r2) * n_dot_o.powi(2)).sqrt();
                    let right =
                        n_dot_o * (r2 + (1.0 - r2) * n_dot_i.powi(2)).sqrt();
                    let geometry = 2.0 * n_dot_i * n_dot_o / (left + right);

                    // Fresnel coefficient (Schlick approximation):
                    let fresnel = f0 + (1.0 - f0) * (1.0 - m_dot_o).powi(5);

                    // Cook-Torrance BRDF:
                    self.color
                        * fresnel
                        * distribution
                        * geometry
                        * tracer.trace(Ray::new(int.position, outgoing))
                } else if !metallic {
                    // Diffuse: Lambert BRDF with cosine sampling.
                    let dir = sample_hemisphere(tracer.rng(), int.normal, 1.0);
                    self.color * tracer.trace(Ray::new(int.position, dir))
                } else {
                    LinSrgb::default()
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

fn sample_ggx<R: Rng + ?Sized>(
    rng: &mut R,
    normal: Vector3,
    roughness: f32,
) -> Vector3 {
    let e = rng.gen::<f32>();
    let theta = (roughness * e.sqrt() / (1.0 - e).sqrt()).atan();
    let phi = rng.gen::<f32>() * 2.0 * PI;
    let dir = Vector3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    );
    (normal.tangent_space() * dir).normalize()
}
