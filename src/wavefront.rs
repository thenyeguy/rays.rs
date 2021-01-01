use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use palette::LinSrgb;
use regex::Regex;

use crate::material::Material;
use crate::object::Object;
use crate::surface::Triangle;
use crate::types::Point3;

static DEFAULT_MATERIAL: &str = "__rays_default__";

#[derive(Clone, Debug)]
pub struct WavefrontObject {
    pub faces: Vec<WavefrontFace>,
    pub materials: HashMap<String, WavefrontMaterial>,
}

impl WavefrontObject {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path: &Path = path.as_ref();

        let face_re =
            Regex::new(r"^f\s+(\S+)\s+(\S+)\s+(\S+)(?: +(\S+))?").unwrap();
        let material_re = Regex::new(r"^usemtl\s+(\S+)").unwrap();
        let material_file_re = Regex::new(r"^mtllib\s+(\S+)").unwrap();
        let vertex_re = Regex::new(r"^v\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let mut material: String = DEFAULT_MATERIAL.into();
        let mut materials = HashMap::new();

        let mut default_material = WavefrontMaterial::new(DEFAULT_MATERIAL);
        default_material.diffuse_color = LinSrgb::new(1.0, 1.0, 1.0);
        materials.insert(DEFAULT_MATERIAL.into(), default_material);

        let file = File::open(path)?;
        for line in BufReader::new(&file).lines() {
            let line = line?;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(caps) = vertex_re.captures(&line) {
                let v1 = caps[1].parse().unwrap_or(0.0);
                let v2 = caps[2].parse().unwrap_or(0.0);
                let v3 = caps[3].parse().unwrap_or(0.0);
                vertices.push(Point3::new(v1, v2, v3));
            } else if let Some(caps) = face_re.captures(&line) {
                let v1 = parse_vertex_index(&caps[1], vertices.len());
                let v2 = parse_vertex_index(&caps[2], vertices.len());
                let v3 = parse_vertex_index(&caps[3], vertices.len());
                faces.push(WavefrontFace {
                    material: material.clone(),
                    vertices: [vertices[v1], vertices[v2], vertices[v3]],
                });

                if let Some(m4) = caps.get(4) {
                    let v4 = parse_vertex_index(m4.as_str(), vertices.len());
                    faces.push(WavefrontFace {
                        material: material.clone(),
                        vertices: [vertices[v1], vertices[v3], vertices[v4]],
                    });
                }
            } else if let Some(caps) = material_file_re.captures(&line) {
                let material_file: &Path = caps[1].as_ref();
                let material_path = path.parent().unwrap().join(material_file);
                WavefrontMaterial::load_materials(
                    material_path,
                    &mut materials,
                )?;
            } else if let Some(caps) = material_re.captures(&line) {
                material = caps[1].into();
            }
        }

        Ok(WavefrontObject { faces, materials })
    }

    pub fn into_objects(self) -> Vec<Object> {
        let mut objects = Vec::new();
        for face in self.faces {
            objects.push(Object::new(
                Triangle::new(face.vertices),
                self.materials[&face.material].as_rays_material(),
            ));
        }
        objects
    }
}

#[derive(Clone, Debug)]
pub struct WavefrontFace {
    pub material: String,
    pub vertices: [Point3; 3],
}

#[derive(Clone, Debug)]
pub struct WavefrontMaterial {
    pub name: String,
    pub diffuse_color: LinSrgb,
    pub emissive_color: LinSrgb,
    pub specular_color: LinSrgb,
}

impl WavefrontMaterial {
    fn new(name: &str) -> Self {
        WavefrontMaterial {
            name: name.into(),
            diffuse_color: LinSrgb::default(),
            emissive_color: LinSrgb::default(),
            specular_color: LinSrgb::default(),
        }
    }

    fn load_materials<P: AsRef<Path>>(
        path: P,
        materials: &mut HashMap<String, Self>,
    ) -> io::Result<()> {
        let new_material_re = Regex::new(r"^newmtl +(\S+)").unwrap();
        let diffuse_color_re =
            Regex::new(r"^ *Ka +(\S+) +(\S+) +(\S+)").unwrap();
        let emissive_color_re =
            Regex::new(r"^ *Ke +(\S+) +(\S+) +(\S+)").unwrap();
        let specular_color_re =
            Regex::new(r"^ *Ks +(\S+) +(\S+) +(\S+)").unwrap();

        let mut material = String::from(DEFAULT_MATERIAL);
        let file = File::open(path.as_ref())?;
        for line in BufReader::new(&file).lines() {
            let line = line?;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some(caps) = new_material_re.captures(&line) {
                material = caps[1].into();
                materials.insert(
                    material.clone(),
                    WavefrontMaterial::new(&material),
                );
            } else if let Some(caps) = diffuse_color_re.captures(&line) {
                let r = caps[1].parse().unwrap_or(0.0);
                let g = caps[2].parse().unwrap_or(0.0);
                let b = caps[3].parse().unwrap_or(0.0);
                materials.get_mut(&material).unwrap().diffuse_color =
                    LinSrgb::new(r, g, b);
            } else if let Some(caps) = emissive_color_re.captures(&line) {
                let r = caps[1].parse().unwrap_or(0.0);
                let g = caps[2].parse().unwrap_or(0.0);
                let b = caps[3].parse().unwrap_or(0.0);
                materials.get_mut(&material).unwrap().emissive_color =
                    LinSrgb::new(r, g, b);
            } else if let Some(caps) = specular_color_re.captures(&line) {
                let r = caps[1].parse().unwrap_or(0.0);
                let g = caps[2].parse().unwrap_or(0.0);
                let b = caps[3].parse().unwrap_or(0.0);
                materials.get_mut(&material).unwrap().specular_color =
                    LinSrgb::new(r, g, b);
            }
        }

        Ok(())
    }

    fn as_rays_material(&self) -> Material {
        if is_color_set(&self.emissive_color) {
            Material::light(self.emissive_color)
        } else if is_color_set(&self.specular_color) {
            Material::specular(self.specular_color)
        } else {
            Material::diffuse(self.diffuse_color)
        }
    }
}

fn parse_vertex_index(index: &str, num_vertices: usize) -> usize {
    let relative_index: isize = index.parse().unwrap();
    if relative_index > 0 {
        (relative_index - 1) as usize
    } else {
        (num_vertices as isize + relative_index) as usize
    }
}

fn is_color_set(color: &LinSrgb) -> bool {
    color.red > 0.0 || color.green > 0.0 || color.blue > 0.0
}
