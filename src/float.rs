use std::cmp::Ordering;

pub const EPSILON: f32 = 1e-5;

pub fn compare(f1: &f32, f2: &f32) -> Ordering {
    f1.partial_cmp(f2).unwrap()
}

pub fn min(f1: f32, f2: f32) -> f32 {
    f1.min(f2)
}

pub fn max(f1: f32, f2: f32) -> f32 {
    f1.max(f2)
}
