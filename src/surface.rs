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

pub trait Surface where Self: Debug {
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
        let c = (ray.origin - self.center).norm_squared() -
                self.radius * self.radius;
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
    plane: Plane,
    vertices: [Point3<f32>; 3],
    edges: [Vector3<f32>; 3],
}

impl Triangle {
    pub fn new(vertices: [Point3<f32>; 3]) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[1];
        let e3 = vertices[0] - vertices[2];
        let normal = e1.cross(&e2);
        Triangle {
            plane: Plane::new(vertices[0], normal),
            vertices: vertices,
            edges: [e1, e2, e3],
        }
    }
}

impl Surface for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        match self.plane.intersect(ray) {
            Some(hit) => {
                let v1 = hit.pos - self.vertices[0];
                let v2 = hit.pos - self.vertices[1];
                let v3 = hit.pos - self.vertices[2];
                if self.plane.normal.dot(&self.edges[0].cross(&v1)) > 0.0 &&
                   self.plane.normal.dot(&self.edges[1].cross(&v2)) > 0.0 &&
                   self.plane.normal.dot(&self.edges[2].cross(&v3)) > 0.0 {
                    Some(hit)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
