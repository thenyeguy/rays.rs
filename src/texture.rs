use palette::{LinSrgb, Srgb};
use std::fmt::Debug;
use std::ops::{Add, Index, Mul};

#[derive(Copy, Clone, Debug)]
pub struct TextureCoords {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug)]
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<LinSrgb>,
}

impl TextureCoords {
    pub fn new(x: f32, y: f32) -> Self {
        TextureCoords { x, y }
    }
}

impl Default for TextureCoords {
    fn default() -> Self {
        TextureCoords::new(0.0, 0.0)
    }
}

impl Add for TextureCoords {
    type Output = TextureCoords;
    fn add(self, other: TextureCoords) -> TextureCoords {
        TextureCoords::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<TextureCoords> for f32 {
    type Output = TextureCoords;
    fn mul(self, rhs: TextureCoords) -> TextureCoords {
        TextureCoords::new(self * rhs.x, self * rhs.y)
    }
}

impl Texture {
    pub fn from_image(img: image::RgbImage) -> Self {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let pixels = img
            .pixels()
            .map(|image::Rgb(p)| Srgb::from(*p).into_linear())
            .collect();
        Texture {
            width,
            height,
            pixels,
        }
    }
}

impl Index<TextureCoords> for Texture {
    type Output = LinSrgb;

    fn index(&self, coords: TextureCoords) -> &Self::Output {
        // Snap to the previous pixel. An improvement would be to linearly
        // interpolate between the 4 pixels we are between.
        let x = (coords.x * self.width as f32) as usize % self.width;
        let y = ((1.0 - coords.y) * self.height as f32) as usize % self.height;
        &self.pixels[x + y * self.width]
    }
}
