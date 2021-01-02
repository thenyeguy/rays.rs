use crate::material::Material;
use crate::ray::Ray;
use crate::surface::{Intersection, Surface};

#[derive(Copy, Clone, Debug)]
pub struct Sample<'a> {
    pub intersection: Intersection,
    pub material: &'a Material,
}

#[derive(Debug)]
pub struct Object {
    pub surface: Box<dyn Surface>,
    pub material: Material,
}

impl Object {
    pub fn new<S>(surface: S, material: Material) -> Self
    where
        S: 'static + Surface,
    {
        Object {
            surface: Box::new(surface),
            material,
        }
    }

    pub fn sample(&self, ray: Ray) -> Option<Sample> {
        self.surface.intersect(ray).map(|intersection| Sample {
            intersection,
            material: &self.material,
        })
    }
}
