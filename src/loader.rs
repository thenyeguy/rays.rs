use std::error;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

use palette::LinSrgb;

use material::Material;
use object::Object;
use ray::Ray;
use scene::Scene;
use surface::*;

pub fn load_scene<P: AsRef<Path>>(path: P) -> Result<Scene, LoadError> {
    let file = File::open(path)?;
    let scene_prototype: ScenePrototype = serde_yaml::from_reader(&file)?;
    Ok(scene_prototype.into())
}

#[derive(Debug, Deserialize)]
struct ScenePrototype {
    #[serde(default)]
    camera: CameraPrototype,
    #[serde(default)]
    global_illumination: (f32, f32, f32),
    objects: Vec<ObjectPrototype>,
}

impl Into<Scene> for ScenePrototype {
    fn into(self) -> Scene {
        let mut objects = Vec::new();
        for object in self.objects {
            let new_objects: Vec<Object> = object.into();
            objects.extend(new_objects.into_iter());
        }
        Scene {
            global_illumination: LinSrgb::from_components(
                self.global_illumination,
            ),
            camera_ray: self.camera.into(),
            objects: objects,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CameraPrototype {
    pos: [f32; 3],
    dir: [f32; 3],
}

impl Default for CameraPrototype {
    fn default() -> Self {
        CameraPrototype {
            pos: [0.0, 0.0, 0.0],
            dir: [0.0, 0.0, 1.0],
        }
    }
}

impl Into<Ray> for CameraPrototype {
    fn into(self) -> Ray {
        Ray::new(self.pos.into(), self.dir.into())
    }
}

#[derive(Debug, Deserialize)]
struct ObjectPrototype {
    #[serde(flatten)]
    surface: SurfacePrototype,
    material: MaterialPrototype,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum SurfacePrototype {
    Sphere { center: [f32; 3], radius: f32 },
    Plane { center: [f32; 3], normal: [f32; 3] },
    Triangle { vertices: [[f32; 3]; 3] },
    Quadrilateral { vertices: [[f32; 3]; 4] },
}

impl Into<Vec<Object>> for ObjectPrototype {
    fn into(self) -> Vec<Object> {
        let mat: Material = self.material.into();
        match self.surface {
            SurfacePrototype::Sphere { center, radius } => {
                vec![Object::new(Sphere::new(center.into(), radius), mat)]
            }
            SurfacePrototype::Plane { center, normal } => {
                vec![Object::new(Plane::new(center.into(), normal.into()), mat)]
            }
            SurfacePrototype::Triangle { vertices } => vec![Object::new(
                Triangle::new([
                    vertices[0].into(),
                    vertices[1].into(),
                    vertices[2].into(),
                ]),
                mat,
            )],
            SurfacePrototype::Quadrilateral { vertices } => {
                let v0 = vertices[0].into();
                let v1 = vertices[1].into();
                let v2 = vertices[2].into();
                let v3 = vertices[3].into();
                vec![
                    Object::new(Triangle::new([v0, v1, v2]), mat),
                    Object::new(Triangle::new([v0, v2, v3]), mat),
                ]
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct MaterialPrototype {
    #[serde(flatten)]
    kind: MaterialKindPrototype,
    color: (f32, f32, f32),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum MaterialKindPrototype {
    Diffuse,
    Specular,
    Light,
}

impl Into<Material> for MaterialPrototype {
    fn into(self) -> Material {
        let color = LinSrgb::from_components(self.color);
        match self.kind {
            MaterialKindPrototype::Diffuse => Material::diffuse(color),
            MaterialKindPrototype::Specular => Material::specular(color),
            MaterialKindPrototype::Light => Material::light(color),
        }
    }
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Yaml(serde_yaml::Error),
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<serde_yaml::Error> for LoadError {
    fn from(e: serde_yaml::Error) -> Self {
        LoadError::Yaml(e)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoadError::Io(ref err) => write!(f, "IO error: {}", err),
            LoadError::Yaml(ref err) => write!(f, "Yaml error: {}", err),
        }
    }
}

impl error::Error for LoadError {
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LoadError::Io(ref err) => Some(err),
            LoadError::Yaml(ref err) => Some(err),
        }
    }
}
