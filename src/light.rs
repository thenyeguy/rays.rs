use nalgebra::Point3;

pub struct Light {
    pub pos: Point3<f64>,
}

impl Light {
    pub fn new(pos: Point3<f64>) -> Light {
        Light { pos: pos }
    }
}
