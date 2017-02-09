use nalgebra::Point3;
use palette::Rgb;

pub struct Light {
    pub pos: Point3<f64>,
    pub color: Rgb,
}

impl Light {
    pub fn new(pos: Point3<f64>, color: Rgb) -> Light {
        Light {
            pos: pos,
            color: color,
        }
    }
}
