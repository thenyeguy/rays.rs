use palette::Rgb;

#[derive(Debug)]
pub struct Material {
    pub color: Rgb,
}

impl Material {
    pub fn new(color: Rgb) -> Self {
        Material { color: color }
    }
}
