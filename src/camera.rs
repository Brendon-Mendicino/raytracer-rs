use rand::Rng;

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

    pub fn ray_map<F: Fn(&[Ray])>(&self, samples: usize, f: F) {
        let mut rays = (0..samples)
            .map(|_| Ray::new(Vec3::ZERO, Vec3::ZERO))
            .take(samples)
            .collect::<Vec<_>>();

        for h in 0..self.height {
            eprint!("\rLeft: {} ", self.height - h);

            for w in 0..self.width {
                let pixel_center = self.pixel_00
                    + (w as f32 * self.pixel_delta_u)
                    + (h as f32 * self.pixel_delta_v);

                for ray in &mut rays {
                    let ray_center = pixel_center + Self::pixel_sample_offset(self);
                    let dir = ray_center - self.camera_center;
                    *ray = Ray::new(self.camera_center, dir);
                }

                f(&rays);
            }
        }
    }

    #[inline]
    fn pixel_sample_offset(&self) -> Vec3 {
        let x = rand::thread_rng().gen::<f32>() - 0.5;
        let y = rand::thread_rng().gen::<f32>() - 0.5;

        (x * self.pixel_delta_u) + (y * self.pixel_delta_v)
    }
}
