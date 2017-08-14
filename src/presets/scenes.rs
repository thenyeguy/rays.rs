use nalgebra::{Vector3, Point3};
use palette::Rgb;

use material::Material;
use object::Object;
use scene::Scene;
use surface::*;

pub fn sphere_room() -> Scene {
    SceneBuilder::new()
        .global_illumination(White, 0.1)
        .sphere((-3.0, -7.0, 33.0), 3.0, Specular(White))
        .sphere((4.0, -6.0, 30.0), 4.0, Diffuse(White))
        .plane((0.0, 0.0, 40.0), z(), Diffuse(White))
        .plane((-10.0, 0.0, 0.0), x(), Diffuse(Red))
        .plane((10.0, 0.0, 0.0), x(), Diffuse(Blue))
        .plane((0.0, -10.0, 0.0), y(), Diffuse(White))
        .plane((0.0, 10.0, 0.0), y(), Light(White))
        .build()
}

struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    fn new() -> SceneBuilder {
        SceneBuilder {
            scene: Scene {
                objects: Vec::new(),
                global_illumination: Rgb::default(),
            },
        }
    }

    fn global_illumination(mut self, color: Color, intensity: f32) -> Self {
        let color: Rgb = color.into();
        self.scene.global_illumination = color * intensity;
        self
    }

    fn plane(mut self,
             center: (f32, f32, f32),
             normal: (f32, f32, f32),
             mat: Mat)
             -> Self {
        self.scene
            .objects
            .push(Object::new(Plane::new(Point3::new(center.0,
                                                     center.1,
                                                     center.2),
                                         Vector3::new(normal.0,
                                                      normal.1,
                                                      normal.2)),
                              mat.into()));
        self
    }

    fn sphere(mut self,
              center: (f32, f32, f32),
              r: f32,
              material: Mat)
              -> Self {
        self.scene
            .objects
            .push(Object::new(Sphere::new(Point3::new(center.0,
                                                      center.1,
                                                      center.2),
                                          r),
                              material.into()));
        self
    }

    fn build(self) -> Scene {
        self.scene
    }
}

enum Mat {
    Diffuse(Color),
    Specular(Color),
    Light(Color),
}
use self::Mat::*;
impl Into<Material> for Mat {
    fn into(self) -> Material {
        match self {
            Diffuse(color) => Material::diffuse(color.into()),
            Specular(color) => Material::specular(color.into()),
            Light(color) => Material::light(color.into()),
        }
    }
}

enum Color {
    Blue,
    Red,
    White,
}
use self::Color::*;
impl Into<Rgb> for Color {
    fn into(self) -> Rgb {
        match self {
            Blue => Rgb::new(0.1, 0.1, 1.0),
            Red => Rgb::new(1.0, 0.0, 0.0),
            White => Rgb::new(1.0, 1.0, 1.0),
        }
    }
}

fn x() -> (f32, f32, f32) {
    (1.0, 0.0, 0.0)
}
fn y() -> (f32, f32, f32) {
    (0.0, 1.0, 0.0)
}
fn z() -> (f32, f32, f32) {
    (0.0, 0.0, 1.0)
}
