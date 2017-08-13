use palette::Rgb;

use material::{Reflection, Sample};
use object::Object;
use ray::Ray;

const MAX_DEPTH: usize = 5;

pub struct Scene {
    pub objects: Vec<Object>,
    pub global_illumination: Rgb,
}

impl Scene {
    pub fn trace(&self, ray: Ray) -> Rgb {
        self.trace_internal(ray, 0)
    }

    fn trace_internal(&self, ray: Ray, depth: usize) -> Rgb {
        if depth > MAX_DEPTH {
            return self.global_illumination;
        }

        match self.sample(ray) {
            Some(sample) => {
                let reflected = match sample.reflection {
                    Some(Reflection { ray, intensity }) => {
                        self.trace_internal(ray, depth + 1) * intensity
                    }
                    None => Rgb::new(0.0, 0.0, 0.0),
                };
                sample.color * (reflected + sample.emission)
            }
            _ => self.global_illumination,
        }
    }

    fn sample(&self, ray: Ray) -> Option<Sample> {
        self.objects
            .iter()
            .filter_map(|obj| obj.collide(ray))
            .min_by(|left, right| {
                left.intersection
                    .distance
                    .partial_cmp(&right.intersection.distance)
                    .unwrap()
            })
            .map(|collision| collision.sample)
    }
}
