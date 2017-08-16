use nalgebra::{Point3, Unit, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3<f32>,
    pub dir: Unit<Vector3<f32>>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, dir: Vector3<f32>) -> Self {
        Ray {
            origin: origin,
            dir: Unit::new_normalize(dir),
        }
    }

    pub fn towards(src: Point3<f32>, dest: Point3<f32>) -> Self {
        Ray {
            origin: src,
            dir: Unit::new_normalize(dest - src),
        }
    }

    pub fn along(&self, distance: f32) -> Point3<f32> {
        self.origin + self.dir.as_ref() * distance
    }
}
