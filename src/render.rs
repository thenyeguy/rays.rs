use nalgebra::{Dot, Norm};
use palette::Rgb;

use ray::Ray;
use scene::Scene;
use surface::{Intersection, Surface};

pub fn trace(scene: &Scene, ray: Ray) -> Rgb {
    match closest_hit(&scene.surfaces, ray) {
        Some(hit) => {
            let mut color = Rgb::new(0.0, 0.0, 0.0);
            for light in &scene.lights {
                let light_ray = Ray::new(hit.pos, light.pos - hit.pos);
                let max_distance = (light.pos - hit.pos).norm();
                if !occluded(&scene.surfaces, light_ray, max_distance) {
                    let intensity = hit.normal
                        .dot(&light_ray.dir)
                        .max(0.0);
                    color = color +
                            hit.material.color * light.color * intensity as f32;
                }
            }
            color
        }
        _ => Rgb::new(0.0, 0.0, 0.0),
    }
}

fn occluded(surfaces: &[Box<Surface>], ray: Ray, max_distance: f64) -> bool {
    surfaces.iter()
        .filter_map(|s| s.intersection(ray))
        .filter(|i| i.distance < max_distance)
        .next()
        .is_some()
}

fn closest_hit<'a>(surfaces: &'a [Box<Surface>],
                   ray: Ray)
                   -> Option<Intersection<'a>> {
    surfaces.iter()
        .filter_map(|s| s.intersection(ray))
        .min_by(|left, right| {
            left.distance.partial_cmp(&right.distance).unwrap()
        })
}
