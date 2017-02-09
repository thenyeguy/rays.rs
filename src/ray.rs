use nalgebra::{Norm, Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>) -> Self {
        Ray {
            origin: origin,
            dir: dir.normalize(),
        }
    }

    pub fn towards(src: Point3<f32>, dest: Point3<f32>) -> Self {
        Ray {
            origin: src,
            dir: (dest - src).normalize(),
        }
    }

    pub fn along(&self, distance: f32) -> Point3<f32> {
        self.origin + self.dir * distance
    }
}
