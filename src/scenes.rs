use nalgebra::{Vector3, Point3};
use palette::Rgb;

use light::Light;
use material::Material;
use scene::Scene;
use surface::*;

pub fn basic_spheres() -> Scene {
    let white = Rgb::new(1.0, 1.0, 1.0);
    let red = Rgb::new(1.0, 0.0, 0.0);
    let blue = Rgb::new(0.1, 0.1, 1.0);
    let yellow = Rgb::new(1.0, 0.9, 0.4);
    Scene {
        surfaces: vec![Box::new(Sphere::new(Point3::new(0.0, 0.0, 20.0),
                                            2.0,
                                            Material::new(red))),
                       Box::new(Sphere::new(Point3::new(3.0, 1.0, 15.0),
                                            1.0,
                                            Material::new(blue))),
                       Box::new(Plane::new(Point3::new(0.0, 2.0, 0.0),
                                           Vector3::new(0.0, 1.0, 0.0),
                                           Material::new(white)))],
        lights: vec![Light::new(Point3::new(10.0, -1.0, 0.0), yellow)],
        ambient_light: white * 0.02,
    }
}
