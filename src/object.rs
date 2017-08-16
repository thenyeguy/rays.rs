use material::{Material, Sample};
use ray::Ray;
use surface::Surface;

#[derive(Copy,Clone,Debug)]
pub struct Collision {
    pub distance: f32,
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
        self.surface.intersect(ray).map(|ref intersection| {
            Collision {
                distance: intersection.distance,
                sample: self.material.sample(ray, &intersection),
            }
        })
    }
}
