use palette::Rgb;

use material::Material;
use ray::Ray;
use surface::{Intersection, Surface};

#[derive(Copy,Clone,Debug)]
pub struct Collision {
    pub intersection: Intersection,
    pub emittance: Rgb,
}

pub struct Object {
    surface: Box<Surface>,
    material: Material,
}

impl Object {
    pub fn new<S>(surface: S, material: Material) -> Self
        where S: 'static + Surface
    {
        Object {
            surface: Box::new(surface),
            material: material,
        }
    }

    pub fn collide(&self, ray: Ray) -> Option<Collision> {
        self.surface.intersect(ray).map(|intersection| {
            Collision {
                intersection: intersection,
                emittance: self.material.color,
            }
        })
    }
}
