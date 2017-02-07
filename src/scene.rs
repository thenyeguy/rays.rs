use light::Light;
use surface::Surface;

pub struct Scene {
    pub surfaces: Vec<Box<Surface>>,
    pub lights: Vec<Light>,
}
