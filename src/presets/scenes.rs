use nalgebra::{Vector3, Point3};
use palette::Rgb;

use light::Light;
use material::Material;
use object::Object;
use scene::Scene;
use surface::*;

fn plane(point: Point3<f32>, normal: Vector3<f32>, color: Rgb) -> Object {
    Object::new(Plane::new(point, normal), Material::new(color))
}

fn sphere(center: Point3<f32>, radius: f32, color: Rgb) -> Object {
    Object::new(Sphere::new(center, radius), Material::new(color))
}

fn triangle(v1: Point3<f32>,
            v2: Point3<f32>,
            v3: Point3<f32>,
            color: Rgb)
            -> Object {
    Object::new(Triangle::new([v1, v2, v3]), Material::new(color))
}

pub fn basic_spheres() -> Scene {
    let white = Rgb::new(1.0, 1.0, 1.0);
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.1, 0.1, 1.0);
    let yellow = Rgb::new(1.0, 0.9, 0.4);
    Scene {
        objects: vec![sphere(Point3::new(0.0, 0.0, 20.0), 2.0, red),
                      sphere(Point3::new(3.0, 1.0, 15.0), 1.0, blue),
                      plane(Point3::new(0.0, 2.0, 0.0),
                            Vector3::new(0.0, 1.0, 0.0),
                            white)],
        lights: vec![Light::new(Point3::new(10.0, -1.0, 0.0), yellow)],
        global_illumination: white * 0.02,
    }
}

pub fn pyramid() -> Scene {
    let white = Rgb::new(1.0, 1.0, 1.0);
    let yellow = Rgb::new(1.0, 0.9, 0.4);

    // Corners
    let front = Point3::new(0.0, 1.0, 8.0);
    let left = Point3::new(-2.0, 1.0, 10.0);
    let right = Point3::new(2.0, 1.0, 10.0);
    let back = Point3::new(0.0, 1.0, 12.0);
    let top = Point3::new(0.0, -1.0, 10.0);

    Scene {
        objects: vec![triangle(front, left, top, white),
                      triangle(front, right, top, white),
                      triangle(back, left, top, white),
                      triangle(back, right, top, white),
                      plane(Point3::new(0.0, 1.0, 0.0),
                            Vector3::new(0.0, 1.0, 0.0),
                            white)],
        lights: vec![Light::new(Point3::new(4.0, -2.0, 0.0), yellow)],
        global_illumination: white * 0.05,
    }
}

pub fn sphere_in_room() -> Scene {
    let white = Rgb::new(1.0, 1.0, 1.0);
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.1, 0.1, 1.0);
    let yellow = Rgb::new(1.0, 0.9, 0.4);
    Scene {
        objects: vec![sphere(Point3::new(0.0, 6.0, 30.0), 4.0, white),
                      plane(Point3::new(0.0, -10.0, 0.0),
                            Vector3::new(0.0, 1.0, 0.0),
                            white),
                      plane(Point3::new(0.0, 10.0, 0.0),
                            Vector3::new(0.0, 1.0, 0.0),
                            white),
                      plane(Point3::new(0.0, 0.0, 40.0),
                            Vector3::new(0.0, 0.0, 1.0),
                            white),
                      plane(Point3::new(-10.0, 0.0, 0.0),
                            Vector3::new(1.0, 0.0, 0.0),
                            red),
                      plane(Point3::new(10.0, 0.0, 0.0),
                            Vector3::new(1.0, 0.0, 0.0),
                            blue)],
        lights: vec![Light::new(Point3::new(9.0, -1.0, 0.0), yellow)],
        global_illumination: white * 0.05,
    }
}
