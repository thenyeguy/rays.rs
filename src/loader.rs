use std::error;
use std::fmt::{self, Debug};
use std::fs::File;
use std::io;
use std::path::Path;
use std::str::FromStr;

use palette::LinSrgb;
use serde::Deserialize;

use crate::bvh::BoundingVolumeHierarchy;
use crate::camera::Camera;
use crate::material::{Color, Material};
use crate::object::Object;
use crate::profile;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::surface::*;
use crate::texture::{Texture, TextureCoords};
use crate::types::{Point3, Vector3};

pub fn load_scene<P: AsRef<Path>>(path: P) -> Result<Scene, LoadError> {
    profile::start("load.prof");
    let root: &Path = path.as_ref();
    let file = File::open(root.join("scene.yaml"))?;
    let scene_prototype: ScenePrototype = serde_yaml::from_reader(&file)?;
    let scene = scene_prototype.compile(root);
    profile::end();
    scene
}

#[derive(Debug, Deserialize)]
struct ScenePrototype {
    camera: CameraPrototype,
    #[serde(default)]
    global_illumination: (f32, f32, f32),
    objects: Vec<ObjectPrototype>,
}

#[derive(Debug, Deserialize)]
struct CameraPrototype {
    pos: (f32, f32, f32),
    dir: (f32, f32, f32),
    #[serde(default = "default_fov")]
    fov: u32,
}

fn default_fov() -> u32 {
    60
}

#[derive(Debug, Deserialize)]
struct ObjectPrototype {
    #[serde(flatten)]
    surface: SurfacePrototype,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum SurfacePrototype {
    Sphere {
        center: (f32, f32, f32),
        radius: f32,
        material: MaterialPrototype,
    },
    Triangle {
        vertices: [(f32, f32, f32); 3],
        material: MaterialPrototype,
    },
    Quadrilateral {
        vertices: [(f32, f32, f32); 4],
        material: MaterialPrototype,
    },
    Wavefront {
        obj_file: String,
    },
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
    Glossy { index: f32, roughness: f32 },
    Light,
}

impl ScenePrototype {
    fn compile(self, root: &Path) -> Result<Scene, LoadError> {
        let mut objects = Vec::new();
        for object in self.objects {
            let new_objects: Result<Vec<Object>, LoadError> = object.load(root);
            objects.extend(new_objects?.into_iter());
        }
        Ok(Scene {
            camera: self.camera.into(),
            global_illumination: LinSrgb::from_components(
                self.global_illumination,
            ),
            objects: BoundingVolumeHierarchy::new(objects),
        })
    }
}

impl Into<Camera> for CameraPrototype {
    fn into(self) -> Camera {
        Camera::new(Ray::new(self.pos.into(), self.dir.into()), self.fov)
    }
}

impl ObjectPrototype {
    fn load(self, root: &Path) -> Result<Vec<Object>, LoadError> {
        let mut objects = Vec::new();
        match self.surface {
            SurfacePrototype::Sphere {
                center,
                radius,
                material,
            } => {
                objects.push(Object::new(
                    Sphere::new(center.into(), radius),
                    material.into(),
                ));
            }
            SurfacePrototype::Triangle { vertices, material } => {
                objects.push(Object::new(
                    Triangle::new(
                        [
                            vertices[0].into(),
                            vertices[1].into(),
                            vertices[2].into(),
                        ],
                        None,
                        None,
                    ),
                    material.into(),
                ));
            }
            SurfacePrototype::Quadrilateral { vertices, material } => {
                let v0 = vertices[0].into();
                let v1 = vertices[1].into();
                let v2 = vertices[2].into();
                let v3 = vertices[3].into();
                let mat: Material = material.into();
                objects.push(Object::new(
                    Triangle::new([v0, v1, v2], None, None),
                    mat.clone(),
                ));
                objects.push(Object::new(
                    Triangle::new([v0, v2, v3], None, None),
                    mat,
                ));
            }
            SurfacePrototype::Wavefront { obj_file } => {
                for object in load_wavefront(&root.join(obj_file))? {
                    objects.push(object);
                }
            }
        }
        Ok(objects)
    }
}

impl Into<Material> for MaterialPrototype {
    fn into(self) -> Material {
        let color = LinSrgb::from_components(self.color);
        match self.kind {
            MaterialKindPrototype::Diffuse => {
                Material::diffuse(Color::solid(color))
            }
            MaterialKindPrototype::Specular => {
                Material::specular(Color::solid(color))
            }
            MaterialKindPrototype::Glossy { index, roughness } => {
                Material::glossy(Color::solid(color), index, roughness)
            }
            MaterialKindPrototype::Light => {
                Material::light(Color::solid(color))
            }
        }
    }
}

fn load_wavefront(path: &Path) -> Result<Vec<Object>, LoadError> {
    let (models, raw_materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: true,
            ignore_lines: true,
            ..Default::default()
        },
    )?;

    let root = path.parent().unwrap();
    let default_material =
        Material::diffuse(Color::solid(LinSrgb::new(1.0, 1.0, 1.0)));
    let materials = raw_materials?
        .iter()
        .map(|m| convert_material(root, m))
        .collect::<Result<Vec<_>, _>>()?;

    let mut objects = Vec::new();
    for model in models.iter() {
        let mesh = &model.mesh;
        let material = mesh
            .material_id
            .map(|idx| materials.get(idx))
            .flatten()
            .unwrap_or(&default_material);
        for i in (0..mesh.indices.len()).step_by(3) {
            let v = |j| mesh_vertex(mesh, mesh.indices[j] as usize);
            let n = |j| mesh_normal(mesh, mesh.indices[j] as usize);
            let t = |j| mesh_texture(mesh, mesh.indices[j] as usize);
            let vertices = [v(i), v(i + 1), v(i + 2)];
            let normals = if mesh.normals.is_empty() {
                None
            } else {
                Some([n(i), n(i + 1), n(i + 2)])
            };
            let texture_coords = if mesh.texcoords.is_empty() {
                None
            } else {
                Some([t(i), t(i + 1), t(i + 2)])
            };
            let surface = Triangle::new(vertices, normals, texture_coords);
            objects.push(Object::new(surface, material.clone()));
        }
    }
    Ok(objects)
}

fn mesh_vertex(mesh: &tobj::Mesh, i: usize) -> Point3 {
    Point3::new(
        mesh.positions[3 * i],
        mesh.positions[3 * i + 1],
        mesh.positions[3 * i + 2],
    )
}

fn mesh_normal(mesh: &tobj::Mesh, i: usize) -> Vector3 {
    Vector3::new(
        mesh.normals[3 * i],
        mesh.normals[3 * i + 1],
        mesh.normals[3 * i + 2],
    )
}

fn mesh_texture(mesh: &tobj::Mesh, i: usize) -> TextureCoords {
    // Some obj files I've used seem to have incomplete texture coordinates, so
    // let's validate each index to paint over that problem.
    let x = mesh.texcoords.get(2 * i).cloned().unwrap_or(0.0);
    let y = mesh.texcoords.get(2 * i + 1).cloned().unwrap_or(0.0);
    TextureCoords::new(x, y)
}

fn convert_material(
    root: &Path,
    m: &tobj::Material,
) -> Result<Material, LoadError> {
    let emissive = emissive_color(&m);
    if color_power(&emissive) > 0.0 {
        return Ok(Material::light(to_color(&emissive)));
    }

    let roughness = (2.0 / (2.0 + m.shininess)).sqrt();
    if m.illumination_model == Some(7) {
        Ok(Material::transparent(
            to_color(&m.specular),
            m.optical_density,
            roughness,
        ))
    } else if !m.diffuse_texture.is_empty() {
        let texture_path = root.join(&m.diffuse_texture);
        let img = image::open(texture_path)?.into_rgb8();
        let color = Color::texture(Texture::from_image(img));
        Ok(Material::glossy(color, m.optical_density, roughness))
    } else if color_power(&m.specular) > 5.0 * color_power(&m.diffuse) {
        Ok(Material::metallic(
            to_color(&m.specular),
            m.optical_density,
            roughness,
        ))
    } else {
        Ok(Material::glossy(
            to_color(&m.diffuse),
            m.optical_density,
            roughness,
        ))
    }
}

fn emissive_color(material: &tobj::Material) -> [f32; 3] {
    let black = [0.0; 3];
    material
        .unknown_param
        .get("Ke")
        .map_or(black, |ke| parse_triple(ke).unwrap_or(black))
}

fn parse_triple<T: Default + FromStr>(s: &str) -> Option<[T; 3]> {
    let mut result = [T::default(), T::default(), T::default()];
    for (i, v) in s.split_whitespace().enumerate().take(3) {
        match v.parse() {
            Ok(f) => result[i] = f,
            _ => return None,
        }
    }
    Some(result)
}

fn to_color(c: &[f32; 3]) -> Color {
    Color::solid(LinSrgb::new(c[0], c[1], c[2]))
}

fn color_power(color: &[f32; 3]) -> f32 {
    color[0].powi(2) + color[1].powi(2) + color[2].powi(2)
}

#[derive(Debug)]
pub enum LoadError {
    ImageError(image::ImageError),
    Io(io::Error),
    Wavefront(tobj::LoadError),
    Yaml(serde_yaml::Error),
}

impl From<image::ImageError> for LoadError {
    fn from(e: image::ImageError) -> Self {
        LoadError::ImageError(e)
    }
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

impl From<tobj::LoadError> for LoadError {
    fn from(e: tobj::LoadError) -> Self {
        LoadError::Wavefront(e)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LoadError::ImageError(ref err) => write!(f, "Image error: {}", err),
            LoadError::Io(ref err) => write!(f, "IO error: {}", err),
            LoadError::Wavefront(ref err) => {
                write!(f, "Wavefront error: {}", err)
            }
            LoadError::Yaml(ref err) => write!(f, "Yaml error: {}", err),
        }
    }
}

impl error::Error for LoadError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            LoadError::ImageError(ref err) => Some(err),
            LoadError::Io(ref err) => Some(err),
            LoadError::Wavefront(ref err) => Some(err),
            LoadError::Yaml(ref err) => Some(err),
        }
    }
}
