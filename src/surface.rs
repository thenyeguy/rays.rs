use nalgebra::{Dot, Norm, Point3, Vector3};

use material::Material;
use ray::Ray;

const EPSILON: f64 = 0.00001;

#[derive(Copy,Clone,Debug)]
pub struct Intersection<'a> {
    pub distance: f64,
    pub pos: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a Material,
}

pub trait Surface {
    fn intersection(&self, ray: Ray) -> Option<Intersection>;
}


pub struct Plane {
    point: Point3<f64>,
    normal: Vector3<f64>,
    material: Material,
}

impl Plane {
    pub fn new(point: Point3<f64>,
               normal: Vector3<f64>,
               material: Material)
               -> Self {
        Plane {
            point: point,
            normal: normal.normalize(),
            material: material,
        }
    }
}

impl Surface for Plane {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
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
                material: &self.material,
            })
        }
    }
}


pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64, material: Material) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Surface for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
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
                material: &self.material,
            })
        }
    }
}
