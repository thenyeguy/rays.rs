use nalgebra::{Point3, Vector3};
use std::fmt::Debug;

use bounds::BoundingBox;
use float;
use ray::Ray;

const EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub pos: Point3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Surface: Debug + Sync {
    fn bounding_box(&self) -> BoundingBox;
    fn centroid(&self) -> Point3<f32>;
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Surface for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::axis_aligned(
            self.center[0] - self.radius,
            self.center[0] + self.radius,
            self.center[1] - self.radius,
            self.center[1] + self.radius,
            self.center[2] - self.radius,
            self.center[2] + self.radius,
        )
    }

    fn centroid(&self) -> Point3<f32> {
        self.center
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // Find the discriminant
        let b = (ray.origin - self.center).dot(&ray.dir) * 2.0;
        let c = (ray.origin - self.center).norm_squared()
            - self.radius * self.radius;
        let dis = b * b - 4.0 * c;

        // If the discriminant is negative, then no intersection exists.
        // Otherwise, solve the quadratic
        if dis < 0.0 {
            None
        } else {
            let distance = (-b - dis.sqrt()) / 2.0;
            // Distance threshold to prevent self-intersection
            if distance <= EPSILON {
                return None;
            }
            let pos = ray.along(distance);
            Some(Intersection {
                distance: distance,
                pos: pos,
                normal: (pos - self.center).normalize(),
            })
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    vertex: Point3<f32>,
    edge1: Vector3<f32>,
    edge2: Vector3<f32>,
    normal: Vector3<f32>,
}

impl Triangle {
    pub fn new(vertices: [Point3<f32>; 3]) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        Triangle {
            vertex: vertices[0],
            edge1: e1,
            edge2: e2,
            normal: e1.cross(&e2).normalize(),
        }
    }
}

impl Surface for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        let v1 = self.vertex;
        let v2 = self.vertex + self.edge1;
        let v3 = self.vertex + self.edge2;
        let (xmin, xmax) = float_bounds(&[v1[0], v2[0], v3[0]]);
        let (ymin, ymax) = float_bounds(&[v1[1], v2[1], v3[1]]);
        let (zmin, zmax) = float_bounds(&[v1[2], v2[2], v3[2]]);
        BoundingBox::axis_aligned(xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn centroid(&self) -> Point3<f32> {
        self.vertex + (self.edge1 + self.edge2) / 3.0
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let pvec = ray.dir.cross(&self.edge2);
        let det = self.edge1.dot(&pvec);
        if det.abs() < EPSILON {
            // Ray is parallel to plane.
            return None;
        }
        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertex;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(&self.edge1);
        let v = qvec.dot(&ray.dir) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let dist = self.edge2.dot(&qvec) * inv_det;
        if dist < EPSILON {
            return None;
        }

        Some(Intersection {
            distance: dist,
            pos: ray.along(dist),
            normal: if det > 0.0 { self.normal } else { -self.normal },
        })
    }
}

fn float_bounds(fs: &[f32]) -> (f32, f32) {
    let min = fs.iter().cloned().min_by(float::compare).unwrap();
    let max = fs.iter().cloned().max_by(float::compare).unwrap();
    (min, max)
}
