use std::fmt::Debug;

use crate::bounds::BoundingBox;
use crate::float;
use crate::ray::Ray;
use crate::texture::TextureCoords;
use crate::types::{Point3, Vector3};
use crate::{increment_statistic, statistics};

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub distance: f32,
    pub position: Point3,
    pub incident: Vector3,
    pub normal: Vector3,
    pub texture_coords: TextureCoords,
}

pub trait Surface: Debug + Sync {
    fn bounding_box(&self) -> BoundingBox;
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Surface for Sphere {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::axis_aligned(
            self.center.x() - self.radius,
            self.center.x() + self.radius,
            self.center.y() - self.radius,
            self.center.y() + self.radius,
            self.center.z() - self.radius,
            self.center.z() + self.radius,
        )
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        // Find the discriminant
        let b = (ray.origin - self.center).dot(ray.dir) * 2.0;
        let c = (ray.origin - self.center).norm_squared() - self.radius * self.radius;
        let dis = b * b - 4.0 * c;

        // If the discriminant is negative, then no intersection exists.
        // Otherwise, solve the quadratic
        if dis < 0.0 {
            None
        } else {
            let distance = (-b - dis.sqrt()) / 2.0;
            // Distance threshold to prevent self-intersection
            if distance <= float::EPSILON {
                return None;
            }
            let position = ray.along(distance);
            Some(Intersection {
                distance,
                position,
                incident: ray.dir,
                normal: (position - self.center).normalize(),
                texture_coords: TextureCoords::default(),
            })
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    vertex: Point3,
    edge1: Vector3,
    edge2: Vector3,
    normals: [Vector3; 3],
    texture_coords: [TextureCoords; 3],
}

impl Triangle {
    pub fn new(
        vertices: [Point3; 3],
        normals: Option<[Vector3; 3]>,
        texture_coords: Option<[TextureCoords; 3]>,
    ) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        let normals = normals.unwrap_or([e1.cross(e2).normalize(); 3]);
        let texture_coords = texture_coords.unwrap_or([TextureCoords::default(); 3]);
        Triangle {
            vertex: vertices[0],
            edge1: e1,
            edge2: e2,
            normals,
            texture_coords,
        }
    }
}

impl Surface for Triangle {
    fn bounding_box(&self) -> BoundingBox {
        let v1 = self.vertex;
        let v2 = self.vertex + self.edge1;
        let v3 = self.vertex + self.edge2;
        let (xmin, xmax) = float_bounds(&[v1.x(), v2.x(), v3.x()]);
        let (ymin, ymax) = float_bounds(&[v1.y(), v2.y(), v3.y()]);
        let (zmin, zmax) = float_bounds(&[v1.z(), v2.z(), v3.z()]);
        BoundingBox::axis_aligned(xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        increment_statistic!(statistics::TRIANGLE_TESTS);

        let pvec = ray.dir.cross(self.edge2);
        let det = self.edge1.dot(pvec);
        if det.abs() < float::EPSILON {
            // Ray is parallel to plane.
            return None;
        }
        let inv_det = 1.0 / det;

        let tvec = ray.origin - self.vertex;
        let u = tvec.dot(pvec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let qvec = tvec.cross(self.edge1);
        let v = qvec.dot(ray.dir) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let distance = self.edge2.dot(qvec) * inv_det;
        if distance < float::EPSILON {
            return None;
        }

        let w = 1.0 - u - v;
        let normal = (w * self.normals[0] + u * self.normals[1] + v * self.normals[2]).normalize();
        let texture_coords =
            w * self.texture_coords[0] + u * self.texture_coords[1] + v * self.texture_coords[2];

        Some(Intersection {
            distance,
            position: ray.along(distance),
            incident: ray.dir,
            normal,
            texture_coords,
        })
    }
}

fn float_bounds(fs: &[f32]) -> (f32, f32) {
    let min = fs.iter().cloned().min_by(float::compare).unwrap();
    let max = fs.iter().cloned().max_by(float::compare).unwrap();
    (min, max)
}
