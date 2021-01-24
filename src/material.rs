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
    GGX {
        index: f32,
        roughness: f32,
        metallic: bool,
        transparent: bool,
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
        Material {
            color,
            kind: Kind::GGX {
                index,
                roughness,
                metallic: false,
                transparent: false,
            },
        }
    }

    pub fn metallic(color: LinSrgb, index: f32, roughness: f32) -> Self {
        Material {
            color,
            kind: Kind::GGX {
                index,
                roughness,
                metallic: true,
                transparent: false,
            },
        }
    }

    pub fn transparent(color: LinSrgb, index: f32, roughness: f32) -> Self {
        Material {
            color,
            kind: Kind::GGX {
                index,
                roughness,
                metallic: false,
                transparent: true,
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
            Kind::GGX {
                index,
                roughness,
                metallic,
                transparent,
            } => {
                // Importance sample a GGX microfacet, then estimate the Fresnel
                // coefficient.
                let microfacet =
                    sample_ggx(tracer.rng(), int.normal, roughness);
                let f0 = if metallic {
                    (self.color.red + self.color.green + self.color.blue) / 3.0
                } else {
                    ((1.0 - index) / (1.0 + index)).powi(2)
                };
                let m_dot_i = microfacet.dot(int.incident).abs();
                let fresnel = f0 + (1.0 - f0) * (1.0 - m_dot_i).powi(5);

                // Use the Fresnel to determine select the next ray type.
                if tracer.rng().gen::<f32>() < fresnel {
                    // Specular reflection:
                    let outgoing = microfacet.reflect(int.incident);

                    // Validate this ray is visible.
                    let m_dot_o = microfacet.dot(outgoing).abs();
                    let n_dot_o = int.normal.dot(outgoing).abs();
                    if m_dot_o < 0.0 || n_dot_o < 0.0 {
                        return LinSrgb::default();
                    }

                    // Calculate the weight of this ray, including parts of the
                    // distribution function that weren't part of the importance
                    // sampling.
                    let m_dot_n = microfacet.dot(int.normal);
                    let n_dot_i = int.normal.dot(int.incident).abs();
                    let weight = m_dot_i / (n_dot_i * m_dot_n);

                    // Smith correlated shadow masking function:
                    let r2 = roughness.powi(2);
                    let left =
                        n_dot_i * (r2 + (1.0 - r2) * n_dot_o.powi(2)).sqrt();
                    let right =
                        n_dot_o * (r2 + (1.0 - r2) * n_dot_i.powi(2)).sqrt();
                    let geometry = 2.0 * n_dot_i * n_dot_o / (left + right);

                    // Cook-Torrance BRDF:
                    self.color
                        * weight
                        * geometry
                        * tracer.trace(Ray::new(int.position, outgoing))
                } else if transparent {
                    // Refraction:
                    let (ni, no) = if int.normal.dot(int.incident) < 0.0 {
                        (1.0, index)
                    } else {
                        (index, 1.0)
                    };
                    let outgoing =
                        match microfacet.refract(int.incident, ni / no) {
                            Some(o) => o,
                            // Total internal reflection.
                            None => return LinSrgb::default(),
                        };

                    // Calculate the weight of this ray, including parts of the
                    // distribution function that weren't part of the importance
                    // sampling.
                    let m_dot_o = microfacet.dot(outgoing).abs();
                    let weight = 4.0 * m_dot_i * m_dot_o * no.powi(2)
                        / (ni * m_dot_i + no * m_dot_o).powi(2);

                    // Smith correlated shadow masking function:
                    let r2 = roughness.powi(2);
                    let n_dot_o = int.normal.dot(outgoing).abs();
                    let n_dot_i = int.normal.dot(int.incident).abs();
                    let left =
                        n_dot_i * (r2 + (1.0 - r2) * n_dot_o.powi(2)).sqrt();
                    let right =
                        n_dot_o * (r2 + (1.0 - r2) * n_dot_i.powi(2)).sqrt();
                    let geometry = 2.0 * n_dot_i * n_dot_o / (left + right);

                    // Cook-Torrance BRDF:
                    tracer.trace(Ray::new(int.position, outgoing))
                        * weight
                        * geometry
                } else if metallic {
                    // Absorb the light.
                    LinSrgb::default()
                } else {
                    // Diffuse: Lambert BRDF with cosine sampling.
                    let dir = sample_hemisphere(tracer.rng(), int.normal, 1.0);
                    self.color * tracer.trace(Ray::new(int.position, dir))
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
