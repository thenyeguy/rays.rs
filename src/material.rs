use nalgebra::Vector3;
use palette::LinSrgb;
use rand::Rng;
use std::f32::consts::PI;

use ray::Ray;
use surface::Intersection;

#[derive(Copy, Clone, Debug)]
pub enum Sample {
    Emit(LinSrgb),
    Bounce(LinSrgb, Ray),
}

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
            color: color,
            kind: Kind::Emissive,
        }
    }

    pub fn diffuse(color: LinSrgb) -> Self {
        Material {
            color: color,
            kind: Kind::Diffuse,
        }
    }

    pub fn specular(color: LinSrgb) -> Self {
        Material {
            color: color,
            kind: Kind::Specular,
        }
    }

    pub fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray: Ray,
        int: &Intersection,
    ) -> Sample {
        match self.kind {
            Kind::Emissive => Sample::Emit(self.color),
            Kind::Diffuse => {
                // Generate a random direction vector.
                let theta = rng.gen::<f32>() * 2.0 * PI;
                let z = rng.gen::<f32>();
                let zp = (1.0 - z * z).sqrt();
                let dir = Vector3::new(zp * theta.cos(), zp * theta.sin(), z);

                // Ensure we sample only from a hemisphere
                let intensity = dir.dot(&int.normal);
                if intensity < 0.0 {
                    Sample::Bounce(
                        self.color * -intensity,
                        Ray::new(int.pos, -1.0 * dir),
                    )
                } else {
                    Sample::Bounce(
                        self.color * intensity,
                        Ray::new(int.pos, dir),
                    )
                }
            }
            Kind::Specular => {
                let dir = ray.dir - 2.0 * int.normal.dot(&ray.dir) * int.normal;
                Sample::Bounce(self.color, Ray::new(int.pos, dir))
            }
        }
    }
}
