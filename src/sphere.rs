use std::ops::Range;

use crate::{
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit<Ray> for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f32>) -> Option<HitRecord> {
        let oc = r.start - self.center;

        let a = Vec3::dot(r.dir, r.dir);
        let half_b = Vec3::dot(r.dir, oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        let sqrtd = f32::sqrt(discriminant);

        /* Find the nearest root that lies in the range. */
        let mut root = -(half_b + sqrtd) / a;
        if !t_range.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.contains(&root) {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (1.0 / self.radius) * (p - self.center);
        let record = HitRecord::new(p, normal, root, self.material, r);

        Some(record)
    }
}
