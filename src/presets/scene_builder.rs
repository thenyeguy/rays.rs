use nalgebra::{Point3, Vector3};
use palette::Rgb;

use material::Material;
use object::Object;
use scene::Scene;
use surface::*;

pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            scene: Scene {
                objects: Vec::new(),
                global_illumination: Rgb::default(),
            },
        }
    }

    pub fn global_illumination(mut self, color: Color, intensity: f32) -> Self {
        let color: Rgb = color.into();
        self.scene.global_illumination = color * intensity;
        self
    }

    fn object<S>(mut self, surface: S, material: Mat) -> Self
        where S: 'static + Surface + Sync
    {
        self.scene.objects.push(Object::new(surface, material.into()));
        self
    }

    pub fn plane(self,
             center: (f32, f32, f32),
             normal: (f32, f32, f32),
             material: Mat)
             -> Self {
        self.object(Plane::new(Point3::new(center.0, center.1, center.2),
                               Vector3::new(normal.0, normal.1, normal.2)),
                    material.into())
    }

    pub fn triangle(self,
                p1: (f32, f32, f32),
                p2: (f32, f32, f32),
                p3: (f32, f32, f32),
                material: Mat)
                -> Self {
        let p1 = Point3::new(p1.0, p1.1, p1.2);
        let p2 = Point3::new(p2.0, p2.1, p2.2);
        let p3 = Point3::new(p3.0, p3.1, p3.2);
        self.object(Triangle::new([p1, p2, p3]), material.into())
    }

    pub fn quad(self,
            p1: (f32, f32, f32),
            p2: (f32, f32, f32),
            p3: (f32, f32, f32),
            p4: (f32, f32, f32),
            material: Mat)
            -> Self {
        self.triangle(p1, p2, p3, material).triangle(p1, p3, p4, material)
    }

    pub fn sphere(self, center: (f32, f32, f32), r: f32, material: Mat) -> Self {
        self.object(Sphere::new(Point3::new(center.0, center.1, center.2), r),
                    material.into())
    }

    pub fn build(self) -> Scene {
        self.scene
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mat {
    Diffuse(Color),
    Specular(Color),
    Light(Color),
}
impl Into<Material> for Mat {
    fn into(self) -> Material {
        match self {
            Mat::Diffuse(color) => Material::diffuse(color.into()),
            Mat::Specular(color) => Material::specular(color.into()),
            Mat::Light(color) => Material::light(color.into()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Blue,
    Red,
    White,
}
impl Into<Rgb> for Color {
    fn into(self) -> Rgb {
        match self {
            Color::Blue => Rgb::new(0.1, 0.1, 1.0),
            Color::Red => Rgb::new(1.0, 0.0, 0.0),
            Color::White => Rgb::new(1.0, 1.0, 1.0),
        }
    }
}
