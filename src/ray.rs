use crate::types::{Point3, Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vector3) -> Self {
        Ray {
            origin: origin,
            dir: dir.normalize(),
        }
    }

    pub fn towards(src: Point3, dest: Point3) -> Self {
        Ray {
            origin: src,
            dir: (dest - src).normalize(),
        }
    }

    pub fn along(&self, distance: f32) -> Point3 {
        self.origin + self.dir * distance
    }
}
