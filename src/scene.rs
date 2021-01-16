use palette::LinSrgb;

use crate::bvh::BoundingVolumeHierarchy;
use crate::camera::Camera;

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub global_illumination: LinSrgb,
    pub objects: BoundingVolumeHierarchy,
}
