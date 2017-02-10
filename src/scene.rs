use nalgebra::{Dot, Norm};
use palette::Rgb;

use light::Light;
use ray::Ray;
use surface::{Intersection, Surface};

pub struct Scene {
    pub surfaces: Vec<Box<Surface>>,
    pub lights: Vec<Light>,
    pub ambient_light: Rgb,
}

impl Scene {
    pub fn trace(&self, ray: Ray) -> Rgb {
        match self.closest_hit(ray) {
            Some(hit) => {
                let mut color = self.ambient_light * hit.material.color;
                for light in &self.lights {
                    let light_ray = Ray::new(hit.pos, light.pos - hit.pos);
                    let max_distance = (light.pos - hit.pos).norm();
                    if !self.occluded(light_ray, max_distance) {
                        let intensity = hit.normal
                            .dot(&light_ray.dir)
                            .max(0.0);
                        color = color +
                                hit.material.color * light.color * intensity;
                    }
                }
                color
            }
            _ => Rgb::new(0.0, 0.0, 0.0),
        }
    }

    fn closest_hit(&self, ray: Ray) -> Option<Intersection> {
        self.surfaces
            .iter()
            .filter_map(|s| s.intersection(ray))
            .min_by(|left, right| {
                left.distance.partial_cmp(&right.distance).unwrap()
            })
    }

    fn occluded(&self, ray: Ray, max_distance: f32) -> bool {
        self.surfaces
            .iter()
            .filter_map(|s| s.intersection(ray))
            .filter(|i| i.distance < max_distance)
            .next()
            .is_some()
    }
}
