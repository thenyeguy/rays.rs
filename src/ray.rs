use nalgebra::{Norm, Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, dir: Vector3<f64>) -> Self {
        Ray {
            origin: origin,
            dir: dir.normalize(),
        }
    }

    pub fn towards(src: Point3<f64>, dest: Point3<f64>) -> Self {
        Ray {
            origin: src,
            dir: (dest - src).normalize(),
        }
    }

    pub fn along(&self, distance: f64) -> Point3<f64> {
        self.origin + self.dir * distance
    }
}
