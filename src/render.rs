use nalgebra::{Dot, Point3, Vector3};

use light::Light;
use ray::Ray;
use surface::{Intersection, Surface};

const OVERSAMPLES: [f64; 5] = [-0.4, -0.2, 0.0, 0.2, 0.4];

pub fn render_pixel(surfaces: &[Box<Surface>],
                    lights: &[Light],
                    x: f64,
                    y: f64)
                    -> f64 {
    let mut brightness = 0.0;
    for dx in &OVERSAMPLES {
        for dy in &OVERSAMPLES {
            let ray = Ray::new(Point3::new(x + dx, y + dy, 0.0), Vector3::z());
            brightness += render_ray(surfaces, lights, ray);
        }
    }
    brightness / (OVERSAMPLES.len() * OVERSAMPLES.len()) as f64
}

fn render_ray(surfaces: &[Box<Surface>], lights: &[Light], ray: Ray) -> f64 {
    match find_closest_intersection(surfaces, ray) {
        Some(intersection) => {
            let mut brightness = 0.0;
            for light in lights {
                let light_ray = Ray::new(intersection.pos,
                                         light.pos - intersection.pos);
                if !intersects(surfaces, light_ray) {
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
