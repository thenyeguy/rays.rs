use palette::LinSrgb;
use rand::Rng;

use material::Sample;
use object::Object;
use ray::Ray;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub global_illumination: LinSrgb,
    pub camera_ray: Ray,
}

impl Scene {
    pub fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        ray: Ray,
    ) -> Option<Sample> {
        self.objects
            .iter()
            .filter_map(|obj| obj.collide(rng, ray))
            .min_by(|left, right| {
                left.distance.partial_cmp(&right.distance).unwrap()
            }).map(|collision| collision.sample)
    }
}
