use palette::LinSrgb;

use crate::object::Object;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub global_illumination: LinSrgb,
    pub camera_ray: Ray,
}
