use palette::LinSrgb;

use crate::camera::Camera;
use crate::object::Object;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub global_illumination: LinSrgb,
    pub objects: Vec<Object>,
}
