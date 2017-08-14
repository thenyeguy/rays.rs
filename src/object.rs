use material::{Material, Sample};
use ray::Ray;
use surface::{Intersection, Surface};

#[derive(Copy,Clone,Debug)]
pub struct Collision {
    pub intersection: Intersection,
    pub sample: Sample,
}

pub struct Object {
    surface: Box<Surface + Sync>,
    material: Material,
}

impl Object {
    pub fn new<S>(surface: S, material: Material) -> Self
        where S: 'static + Surface + Sync
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
                sample: self.material.sample(ray, intersection),
            }
        })
    }
}
