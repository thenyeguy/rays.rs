use nalgebra::{Norm, Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub pos: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(pos: Point3<f64>, dir: Vector3<f64>) -> Self {
        Ray {
            pos: pos,
            dir: dir.normalize(),
        }
    }

    pub fn towards(src: Point3<f64>, dest: Point3<f64>) -> Self {
        Ray {
            pos: src,
            dir: (dest - src).normalize(),
        }
    }

    pub fn along(&self, distance: f64) -> Point3<f64> {
        self.pos + self.dir * distance
    }
}
