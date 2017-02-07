use image::{ImageBuffer, RgbImage, Rgb};

use scene::Scene;
use render::render_pixel;

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

    pub fn draw(&self, scene: &Scene) -> RgbImage {
        ImageBuffer::from_fn(self.width as u32, self.height as u32, |i, j| {
            let x = i as f64 - (self.width / 2) as f64;
            let y = j as f64 - (self.height / 2) as f64;

            let brightness = render_pixel(scene, x, y);
            let to_color = |f| ((0.8 * f + 0.1) * 255.0) as u8;
            let pixel = to_color(brightness);
            Rgb([pixel, pixel, pixel])
        })
    }
}
