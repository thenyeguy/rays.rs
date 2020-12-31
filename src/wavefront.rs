use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use regex::Regex;

use crate::types::Point3;

#[derive(Clone, Debug, Default)]
pub struct WavefrontObject {
    pub faces: Vec<[Point3; 3]>,
}

impl WavefrontObject {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut vertices: Vec<Point3> = Vec::new();
        let mut faces: Vec<(usize, usize, usize)> = Vec::new();

        let group_re = Regex::new(r"^g +(\w+)").unwrap();
        let vertex_re = Regex::new(r"^v +(\S+) +(\S+) +(\S+)").unwrap();
        let face_re = Regex::new(r"^f +(\d+) +(\d+) +(\d+)").unwrap();

        let file = File::open(path.as_ref())?;
        for line in BufReader::new(&file).lines() {
            let line = line?;
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if group_re.is_match(&line) {
                continue;
            }

            if let Some(caps) = vertex_re.captures(&line) {
                let v1 = caps[1].parse().unwrap_or(0.0);
                let v2 = caps[2].parse().unwrap_or(0.0);
                let v3 = caps[3].parse().unwrap_or(0.0);
                vertices.push(Point3::new(v1, v2, v3));
            } else if let Some(caps) = face_re.captures(&line) {
                let v1: usize = caps[1].parse().unwrap();
                let v2: usize = caps[2].parse().unwrap();
                let v3: usize = caps[3].parse().unwrap();
                faces.push((v1 - 1, v2 - 1, v3 - 1));
            }
        }

        let mut object = WavefrontObject::default();
        for face in faces {
            let v0 = vertices[face.0];
            let v1 = vertices[face.1];
            let v2 = vertices[face.2];
            object.faces.push([v0, v1, v2]);
        }
        Ok(object)
    }
}
