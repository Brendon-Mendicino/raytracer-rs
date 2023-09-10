use std::{
    ops::Range,
    sync::atomic::{self, AtomicU32},
    thread,
};

use rand::Rng;

use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

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

    pub fn ray_map<F: Fn(&[Ray]) -> Color + Sync>(&self, samples: u32, f: F) -> Vec<Vec<Color>> {
        let f = &f;
        let progress = AtomicU32::new(0);
        let progress = &progress;

        thread::scope(|s| {
            let tot_t = thread::available_parallelism().unwrap().get() as u32;

            let mut handles = Vec::<_>::with_capacity(tot_t as usize);

            for curr_t in 0..tot_t {
                let h = s.spawn(move || {
                    self.ray_map_inner(
                        samples,
                        Self::thread_partition(self.height, curr_t, tot_t),
                        progress,
                        f,
                    )
                });

                handles.push(h);
            }

            s.spawn(move || Self::progress_logger(self.height, progress));

            handles
                .into_iter()
                .map(|h| h.join().unwrap())
                .collect::<Vec<_>>()
        })
    }

    fn ray_map_inner<F: Fn(&[Ray]) -> Color + Sync>(
        &self,
        samples: u32,
        h_range: Range<u32>,
        progress: &AtomicU32,
        f: &F,
    ) -> Vec<Color> {
        let mut rays = (0..samples)
            .map(|_| Ray::new(Vec3::ZERO, Vec3::ZERO))
            .take(samples as usize)
            .collect::<Vec<_>>();

        let mut colors = Vec::with_capacity(self.width as usize);

        for h in h_range {
            progress.fetch_add(1, atomic::Ordering::Relaxed);
            for w in 0..self.width {
                let pixel_center = self.pixel_00
                    + (w as f32 * self.pixel_delta_u)
                    + (h as f32 * self.pixel_delta_v);

                for ray in &mut rays {
                    let ray_center = pixel_center + Self::pixel_sample_offset(self);
                    let dir = ray_center - self.camera_center;
                    *ray = Ray::new(self.camera_center, dir);
                }

                colors.push(f(&rays));
            }
        }

        colors
    }

    #[inline]
    fn pixel_sample_offset(&self) -> Vec3 {
        let x = rand::thread_rng().gen::<f32>() - 0.5;
        let y = rand::thread_rng().gen::<f32>() - 0.5;

        (x * self.pixel_delta_u) + (y * self.pixel_delta_v)
    }

    fn thread_partition(max: u32, curr_thread: u32, tot_threads: u32) -> Range<u32> {
        let per_thread = max / tot_threads;
        if curr_thread != tot_threads - 1 {
            (per_thread * curr_thread)..(per_thread * (curr_thread + 1))
        } else {
            (per_thread * curr_thread)..(max)
        }
    }

    fn progress_logger(max: u32, progress: &AtomicU32) {
        let progress_len = 30;

        loop {
            let progress = progress.load(atomic::Ordering::Relaxed);

            eprint!(
                "\r[{}]",
                (0..progress_len)
                    .map(|i| if i < progress_len * progress / max {
                        '#'
                    } else {
                        ' '
                    })
                    .collect::<String>()
            );

            if progress == max {
                break;
            }

            thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
