use std::ops::Range;

use crate::{ray::Ray, vec3::Vec3};

pub trait Hit<F> {
    fn hit(&self, f: &F, t_range: Range<f32>) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f32, ray: &Ray) -> Self {
        let front_face = Vec3::dot(ray.dir, normal) < 0.0;

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}
