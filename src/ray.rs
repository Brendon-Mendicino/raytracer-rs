use crate::vec3::Vec3;

pub struct Ray {
    pub start: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(start: Vec3, dir: Vec3) -> Self {
        Self { start, dir }
    }

    pub fn at(&self, time: f32) -> Vec3 {
        self.start + Vec3::scale(time, self.dir)
    }
}
