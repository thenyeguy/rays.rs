use nalgebra::{Dot, Norm};
use palette::Rgb;

use light::Light;
use object::{Collision, Object};
use ray::Ray;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub global_illumination: Rgb,
}

impl Scene {
    pub fn trace(&self, ray: Ray) -> Rgb {
        match self.closest_hit(ray) {
            Some(Collision { intersection: int, emittance: emit }) => {
                let mut color = self.global_illumination * emit;
                for light in &self.lights {
                    let light_ray = Ray::new(int.pos, light.pos - int.pos);
                    let max_distance = (light.pos - int.pos).norm();
                    if !self.occluded(light_ray, max_distance) {
                        let intensity = int.normal
                            .dot(&light_ray.dir)
                            .max(0.0);
                        color = color + emit * light.color * intensity;
                    }
                }
                color
            }
            _ => Rgb::new(0.0, 0.0, 0.0),
        }
    }

    fn closest_hit(&self, ray: Ray) -> Option<Collision> {
        self.objects
            .iter()
            .filter_map(|obj| obj.collide(ray))
            .min_by(|left, right| {
                left.intersection
                    .distance
                    .partial_cmp(&right.intersection.distance)
                    .unwrap()
            })
    }

    fn occluded(&self, ray: Ray, max_distance: f32) -> bool {
        self.objects
            .iter()
            .filter_map(|obj| obj.collide(ray))
            .filter(|col| col.intersection.distance < max_distance)
            .next()
            .is_some()
    }
}
