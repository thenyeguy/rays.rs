use nalgebra::{Dot, Vector3};
use palette::Rgb;
use rand::{self, Rng};
use std::f32::consts::PI;

use ray::Ray;
use surface::Intersection;

#[derive(Debug,Copy,Clone)]
pub struct Reflection {
    pub ray: Ray,
    pub intensity: f32,
}

#[derive(Debug,Copy,Clone)]
pub struct Sample {
    pub color: Rgb,
    pub emission: f32,
    pub reflection: Option<Reflection>,
}

#[derive(Debug,Copy,Clone)]
enum Kind {
    Emissive,
    Diffuse,
    Specular,
}

#[derive(Debug)]
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

    pub fn sample(&self, ray: Ray, int: &Intersection) -> Sample {
        let reflection = match self.kind {
            Kind::Emissive => None,
            Kind::Diffuse => {
                // Generate a random direction vector.
                let mut rng = rand::thread_rng();
                let theta = rng.gen_range(0.0, 2.0 * PI);
                let z: f32 = rng.gen_range(0.0, 1.0);
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
                let dir = ray.dir - 2.0 * int.normal.dot(&ray.dir) * int.normal;
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
