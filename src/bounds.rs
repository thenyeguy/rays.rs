use crate::float;
use crate::ray::Ray;
use crate::types::Point3;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl BoundingBox {
    pub fn empty() -> Self {
        BoundingBox {
            min: Point3::new(
                std::f32::INFINITY,
                std::f32::INFINITY,
                std::f32::INFINITY,
            ),
            max: Point3::new(
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
            ),
        }
    }

    pub fn axis_aligned(
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> Self {
        BoundingBox {
            min: Point3::new(xmin, ymin, zmin),
            max: Point3::new(xmax, ymax, zmax),
        }
    }

    pub fn union(left: &Self, right: &Self) -> Self {
        BoundingBox {
            min: Point3::new(
                float::min(left.min.x(), right.min.x()),
                float::min(left.min.y(), right.min.y()),
                float::min(left.min.z(), right.min.z()),
            ),
            max: Point3::new(
                float::max(left.max.x(), right.max.x()),
                float::max(left.max.y(), right.max.y()),
                float::max(left.max.z(), right.max.z()),
            ),
        }
    }

    pub fn volume(&self) -> f32 {
        let x = self.max.x() - self.min.x();
        let y = self.max.y() - self.min.y();
        let z = self.max.z() - self.min.z();
        x * y * z
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        let mut tmin = std::f32::NEG_INFINITY;
        let mut tmax = std::f32::INFINITY;
        for i in 0..3 {
            let origin = ray.origin[i];
            let dir = ray.dir[i];
            let t1 = (self.min[i] - origin) / dir;
            let t2 = (self.max[i] - origin) / dir;
            tmin = float::max(tmin, float::min(t1, t2));
            tmax = float::min(tmax, float::max(t1, t2));
        }
        tmin <= tmax && tmax > 0.0
    }
}
