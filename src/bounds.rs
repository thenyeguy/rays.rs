use nalgebra::Point3;

use float;
use ray::Ray;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    min: Point3<f32>,
    max: Point3<f32>,
}

impl BoundingBox {
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

    pub fn intersects(&self, ray: Ray) -> bool {
        let mut tmin = std::f32::NEG_INFINITY;
        let mut tmax = std::f32::INFINITY;
        for i in 0..3 {
            let origin = ray.origin[i];
            let dir = ray.dir[i];
            if dir != 0.0 {
                let t1 = (self.min[i] - origin) / dir;
                let t2 = (self.max[i] - origin) / dir;
                tmin = float::max(tmin, float::min(t1, t2));
                tmax = float::min(tmax, float::max(t1, t2));
            } else if origin < self.min[i] || origin > self.max[i] {
                return false;
            }
        }
        tmin <= tmax && tmax > 0.0
    }
}
