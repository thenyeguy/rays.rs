use nalgebra::{Point3, Vector3};
use std::fmt::Debug;

use ray::Ray;

const EPSILON: f32 = 0.00001;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub pos: Point3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Surface
where
    Self: Debug,
{
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    point: Point3<f32>,
    normal: Vector3<f32>,
}

impl Plane {
    pub fn new(point: Point3<f32>, normal: Vector3<f32>) -> Self {
        Plane {
            point: point,
            normal: normal.normalize(),
        }
    }
}

impl Surface for Plane {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let denom = self.normal.dot(&ray.dir);
        if denom.abs() < EPSILON {
            return None;
        }
        let distance = (self.point - ray.origin).dot(&self.normal) / denom;
        if distance < EPSILON {
            None
        } else {
            Some(Intersection {
                distance: distance,
                pos: ray.along(distance),
                normal: if denom < 0.0 {
                    self.normal
                } else {
                    -self.normal
                },
            })
        }
    }
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
            normal: if det > 0.0 {
                self.normal
            } else {
                -self.normal
            },
        })
    }
}
