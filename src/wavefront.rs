use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use palette::LinSrgb;

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
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut material: String = DEFAULT_MATERIAL.into();
        let mut materials = HashMap::new();
        let mut default_material = WavefrontMaterial::new(DEFAULT_MATERIAL);
        default_material.diffuse_color = LinSrgb::new(1.0, 1.0, 1.0);
        materials.insert(DEFAULT_MATERIAL.into(), default_material);

        let path: &Path = path.as_ref();
        let file = File::open(path)?;
        for line in BufReader::new(&file).lines() {
            let line = line?.trim().replace('\t', " ");
            if line.is_empty() || line.starts_with('#') {
                continue;
            } else if line.starts_with("mtllib") {
                let material_file = line.strip_prefix("mtllib").unwrap().trim();
                let material_path = path.parent().unwrap().join(material_file);
                WavefrontMaterial::load_materials(
                    material_path,
                    &mut materials,
                )?;
            } else if line.starts_with("usemtl") {
                material = line.strip_prefix("usemtl").unwrap().trim().into();
            } else if line.starts_with("v ") {
                vertices.push(Point3::from(collect_triple(&line)?));
            } else if line.starts_with("f ") {
                let indices = collect_args::<FaceIndex>(&line)?;
                if indices.len() >= 3 {
                    let v1 = indices[0].vertex(vertices.len())?;
                    let v2 = indices[1].vertex(vertices.len())?;
                    let v3 = indices[2].vertex(vertices.len())?;
                    faces.push(WavefrontFace {
                        material: material.clone(),
                        vertices: [vertices[v1], vertices[v2], vertices[v3]],
                    });

                    if indices.len() >= 4 {
                        let v4 = indices[3].vertex(vertices.len())?;
                        faces.push(WavefrontFace {
                            material: material.clone(),
                            vertices: [
                                vertices[v1],
                                vertices[v3],
                                vertices[v4],
                            ],
                        });
                    }
                } else {
                    return Err(io::Error::from(io::ErrorKind::InvalidData));
                }
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
        let mut material = String::from(DEFAULT_MATERIAL);
        let file = File::open(path.as_ref())?;
        for line in BufReader::new(&file).lines() {
            let line = line?.trim().replace('\t', " ");
            if line.is_empty() || line.starts_with('#') {
                continue;
            } else if line.starts_with("newmtl") {
                material = line.strip_prefix("newmtl").unwrap().trim().into();
                materials.insert(
                    material.clone(),
                    WavefrontMaterial::new(&material),
                );
            } else if line.starts_with("Kd") {
                materials.get_mut(&material).unwrap().diffuse_color =
                    LinSrgb::from_components(collect_triple(&line)?);
            } else if line.starts_with("Ke") {
                materials.get_mut(&material).unwrap().emissive_color =
                    LinSrgb::from_components(collect_triple(&line)?);
            } else if line.starts_with("Ks") {
                materials.get_mut(&material).unwrap().specular_color =
                    LinSrgb::from_components(collect_triple(&line)?);
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

#[derive(Clone, Debug)]
struct FaceIndex {
    vertex_index: isize,
}

impl FaceIndex {
    fn vertex(&self, num_vertices: usize) -> io::Result<usize> {
        get_absolute_index(self.vertex_index, num_vertices)
    }
}

impl FromStr for FaceIndex {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components = s.split('/').collect::<Vec<_>>();
        let vertex_index: isize = components.get(0).map_or(
            Err(io::Error::from(io::ErrorKind::InvalidData)),
            |idx| parse_arg(idx),
        )?;
        Ok(FaceIndex { vertex_index })
    }
}

fn collect_triple<T: Copy + FromStr>(line: &str) -> io::Result<(T, T, T)> {
    match collect_args(line)?.as_slice() {
        [a, b, c] => Ok((*a, *b, *c)),
        _ => Err(io::Error::from(io::ErrorKind::InvalidData)),
    }
}

fn collect_args<T: FromStr>(line: &str) -> io::Result<Vec<T>> {
    line.split_whitespace()
        .skip(1)
        .map(parse_arg::<T>)
        .collect()
}

fn parse_arg<T: FromStr>(arg: &str) -> io::Result<T> {
    arg.parse::<T>()
        .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))
}

fn get_absolute_index(index: isize, max_index: usize) -> io::Result<usize> {
    if index == 0 || index > max_index as isize {
        Err(io::Error::from(io::ErrorKind::InvalidData))
    } else if index > 0 {
        Ok((index - 1) as usize)
    } else {
        Ok((max_index as isize + index) as usize)
    }
}

fn is_color_set(color: &LinSrgb) -> bool {
    color.red > 0.0 || color.green > 0.0 || color.blue > 0.0
}
