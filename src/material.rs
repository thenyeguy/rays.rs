use nalgebra::Vector3;
use palette::Rgb;
use rand::Rng;
use std::f32::consts::PI;

use ray::Ray;
use surface::Intersection;

#[derive(Copy, Clone, Debug)]
pub struct Reflection {
    pub ray: Ray,
    pub intensity: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Sample {
    pub color: Rgb,
    pub emission: f32,
    pub reflection: Option<Reflection>,
}

#[derive(Copy, Clone, Debug)]
enum Kind {
    Emissive,
    Diffuse,
    Specular,
}

#[derive(Copy, Clone, Debug)]
pub struct Material {
    color: Rgb,
    emittance: f32,
    kind: Kind,
}

impl Material {
    pub fn light(color: Rgb) -> Self {
        Material {
            color: color,
            emittance: 1.0,
            kind: Kind::Emissive,
        }
    }

    pub fn diffuse(color: Rgb) -> Self {
        Material {
            color: color,
            emittance: 0.0,
            kind: Kind::Diffuse,
        }
    }

    pub fn specular(color: Rgb) -> Self {
        Material {
            color: color,
            emittance: 0.0,
            kind: Kind::Specular,
        }
    }

    pub fn sample(&self, rng: &mut Rng, ray: Ray, int: &Intersection) -> Sample {
        let reflection = match self.kind {
            Kind::Emissive => None,
            Kind::Diffuse => {
                // Generate a random direction vector.
                let theta = rng.next_f32() * 2.0 * PI;
                let z: f32 = rng.next_f32();
                let zp = (1.0 - z * z).sqrt();
                let dir = Vector3::new(zp * theta.cos(), zp * theta.sin(), z);

                // Ensure we sample only from a hemisphere
                let intensity = dir.dot(&int.normal);
                if intensity < 0.0 {
                    Some(Reflection {
                        ray: Ray::new(int.pos, -1.0 * dir),
                        intensity: -intensity,
                    })
                } else {
                    Some(Reflection {
                        ray: Ray::new(int.pos, dir),
                        intensity: intensity,
                    })
                }
            }
            Kind::Specular => {
                let ray_dir = ray.dir.as_ref();
                let normal = int.normal.as_ref();
                let dir = ray_dir - 2.0 * normal.dot(ray_dir) * normal;
                Some(Reflection {
                    ray: Ray::new(int.pos, dir),
                    intensity: 1.0,
                })
            }
        };
        Sample {
            color: self.color,
            emission: self.emittance,
            reflection: reflection,
        }
    }
}
