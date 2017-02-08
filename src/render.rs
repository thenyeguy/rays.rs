use nalgebra::Dot;

use ray::Ray;
use scene::Scene;
use surface::{Intersection, Surface};

pub fn render_ray(scene: &Scene, ray: Ray) -> f64 {
    match find_closest_intersection(&scene.surfaces, ray) {
        Some(intersection) => {
            let mut brightness = 0.0;
            for light in &scene.lights {
                let light_ray = Ray::new(intersection.pos,
                                         light.pos - intersection.pos);
                if !intersects(&scene.surfaces, light_ray) {
                    brightness += intersection.normal
                        .dot(&light_ray.dir)
                        .max(0.0);
                }
            }
            brightness
        }
        _ => 0.0,
    }
}


fn intersects(surfaces: &[Box<Surface>], ray: Ray) -> bool {
    surfaces.iter().filter_map(|s| s.intersection(ray)).next().is_some()
}

fn find_closest_intersection(surfaces: &[Box<Surface>],
                             ray: Ray)
                             -> Option<Intersection> {
    let mut min_int = None;
    for int in surfaces.iter().filter_map(|s| s.intersection(ray)) {
        match min_int {
            None => min_int = Some(int),
            Some(int2) => {
                if int.distance < int2.distance {
                    min_int = Some(int)
                }
            }
        }
    }
    min_int
}
