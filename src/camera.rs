use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::{Point3, Vector3};

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

    pub fn draw(&self, surfaces: &Vec<Box<Surface>>) -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f64 - (self.width / 2) as f64;
            let y = j as f64 - (self.width / 2) as f64;
            let ray = Ray::new(Point3::new(x, y, 0.0), Vector3::z());

            // Map to white for a hit, black for a miss
            match find_closest_intersection(surfaces, ray) {
                Some((_, intersection)) => {
                    let to_color = |f| (0.5 * f + 0.5) * 255.0;
                    let red = to_color(intersection.normal.x);
                    let green = to_color(intersection.normal.y);
                    let blue = to_color(intersection.normal.z);
                    Rgb([red as u8, green as u8, blue as u8])
                }
                None => Rgb([0, 0, 0]),
            }
        })
    }
}

fn find_closest_intersection(surfaces: &Vec<Box<Surface>>,
                             ray: Ray)
                             -> Option<(usize, Intersection)> {
    let mut min_int = None;
    for (i, surface) in surfaces.iter().enumerate() {
        match surface.intersection(ray) {
            Some(int) => {
                match min_int {
                    None => min_int = Some((i, int)),
                    Some((_, int2)) => {
                        if int.distance < int2.distance {
                            min_int = Some((i, int))
                        }
                    }
                }
            }
            None => (),
        }
    }
    min_int
}
