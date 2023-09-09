use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    width: u32,
    height: u32,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_00: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, width: u32) -> Self {
        let height = (width as f32 / aspect_ratio) as u32;

        let focal_length = 1.0f32;
        let viewport_h = 2.0f32;
        let viewport_w = viewport_h * (width as f32 / height as f32);
        let camera_center = Vec3::new((0.0, 0.0, 0.0));

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new((viewport_w, 0.0, 0.0));
        let viewport_v = Vec3::new((0.0, -viewport_h, 0.0));

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = (1.0 / width as f32) * viewport_u;
        let pixel_delta_v = (1.0 / height as f32) * viewport_v;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - Vec3::new((0.0, 0.0, focal_length)) - 0.5 * (viewport_u + viewport_v);
        let pixel_00 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            width,
            height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00,
        }
    }

    pub fn ray_iter(&self) -> RayIter {
        RayIter {
            curr_h: 0,
            curr_w: 0,
            height: self.height,
            width: self.width,
            pixel_00: self.pixel_00,
            camera_center: self.camera_center,
            pixel_delta_u: self.pixel_delta_u,
            pixel_delta_v: self.pixel_delta_v,
        }
    }
}

pub struct RayIter {
    curr_h: u32,
    curr_w: u32,
    height: u32,
    width: u32,
    pixel_00: Vec3,
    camera_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Iterator for RayIter {
    type Item = Ray;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_w == self.width {
            self.curr_w = 0;
            self.curr_h += 1;

            if self.curr_h == self.height {
                return None;
            }
        }

        let pixel_center = self.pixel_00
            + (self.curr_w as f32 * self.pixel_delta_u)
            + (self.curr_h as f32 * self.pixel_delta_v);
        let ray_direction = pixel_center - self.camera_center;
        let r = Ray::new(self.camera_center, ray_direction);

        self.curr_w += 1;

        Some(r)
    }
}
