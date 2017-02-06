use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::{Dot, Point3, Vector3};

use light::Light;
use ray::Ray;
use surface::{Intersection, Surface};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    aspect_ratio: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Camera {
            width: width,
            height: height,
            aspect_ratio: height as f64 / width as f64,
        }
    }

    pub fn draw(&self,
                surfaces: &[Box<Surface>],
                lights: &[Light])
                -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f64 - (self.width / 2) as f64;
            let y = j as f64 - (self.height / 2) as f64;
            let ray = Ray::new(Point3::new(x, y, 0.0), Vector3::z());

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
                    let to_color = |f| ((0.8 * f + 0.1) * 255.0) as u8;
                    let pixel = to_color(brightness);
                    Rgb([pixel, pixel, pixel])
                }
                None => Rgb([0, 0, 0]),
            }
        })
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
