use nalgebra::{Dot, Norm, Point3, Vector3};

use ray::Ray;

#[derive(Copy,Clone,Debug)]
pub struct Intersection {
    pub distance: f64,
    pub pos: Point3<f64>,
    pub normal: Vector3<f64>,
}

pub trait Surface {
    fn intersection(&self, ray: Ray) -> Option<Intersection>;
}


#[derive(Copy,Clone,Debug)]
pub struct Sphere {
    pub center: Point3<f64>,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Surface for Sphere {
    fn intersection(&self, ray: Ray) -> Option<Intersection> {
        // Find the discriminant
        let b = (ray.pos - self.center).dot(&ray.dir) * 2.0;
        let c = (ray.pos - self.center).norm_squared() -
                self.radius * self.radius;
        let dis = b * b - 4.0 * c;

        // If the discriminant is negative, then no intersection exists.
        // Otherwise, solve the quadratic
        if dis < 0.0 {
            None
        } else {
            let distance = (-b - dis.sqrt()) / 2.0;
            // Distance threshold to prevent self-intersection
            if distance <= 0.00001 {
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


#[cfg(test)]
mod tests {
    use nalgebra::{approx_eq, Point3, Vector3};
    use num::Float;
    use super::*;
    use ray::Ray;

    #[test]
    fn test_sphere_hits() {
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0),
                           Vector3::new(1.0, 1.0, 1.0));
        let intersection = sphere.intersection(ray).expect("no intersection");
        assert!(approx_eq(&intersection.distance, &(3.0.sqrt() - 1.0)));
    }

    #[test]
    fn test_sphere_tangent() {
        let sphere = Sphere::new(Point3::new(1.0, 0.0, 1.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0),
                           Vector3::new(1.0, 0.0, 0.0));
        let intersection = sphere.intersection(ray).expect("no intersection");
        assert!(approx_eq(&intersection.distance, &1.0));
    }

    #[test]
    fn test_sphere_misses() {
        let sphere = Sphere::new(Point3::new(1.0, 1.0, 1.0), 1.0);
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0),
                           Vector3::new(-1.0, 1.0, 1.0));
        assert!(sphere.intersection(ray).is_none());
    }
}
