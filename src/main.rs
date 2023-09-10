use hit::Hit;
use sphere::Sphere;

use crate::camera::Camera;
use crate::vec3::{Color, Vec3};

use crate::ray::Ray;

mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let dir = Vec3::rand_unit();

    if Vec3::dot(dir, normal) < 0.0 {
        -dir
    } else {
        dir
    }
}

fn ray_color(r: Ray, world: &[Sphere], intensity: f32, depth: u32) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    let mut max_t = f32::INFINITY;

    let hit = world
        .iter()
        .filter_map(|s| match s.hit(&r, 0.001..max_t) {
            Some(hit) => {
                max_t = hit.t;
                Some(hit)
            }
            None => None,
        })
        .last();

    if let Some(hit) = hit {
        let dir = random_on_hemisphere(hit.normal);
        return ray_color(Ray::new(hit.p, dir), world, intensity * 0.5, depth - 1);
    }

    let dir = Vec3::unit(r.dir);
    let a = 0.5 * (dir.y() + 1.0);

    intensity * ((1.0 - a) * Color::new((1.0, 1.0, 1.0)) + a * Color::new((0.5, 0.7, 1.0)))
}

fn main() {
    let aspect_ratio = 19.0 / 9.0 as f32;
    let width = 1980u32;
    let height = (width as f32 / aspect_ratio) as u32;

    let camera = Camera::new(aspect_ratio, width);

    let world = vec![
        Sphere::new(Vec3::new((0.0, 0.0, -1.0)), 0.5),
        Sphere::new(Vec3::new((0.0, -100.5, -1.0)), 100.0),
    ];

    print!("P3\n{} {}\n255\n", width, height);

    let samples = 20;

    camera.ray_map(samples, |r| {
        let mut pixel_color = r
            .iter()
            .map(|r| ray_color(*r, &world, 1.0, 25))
            .fold(Color::BLACK, |a, b| a + b);

        pixel_color = (1.0 / samples as f32) * pixel_color;

        println!("{}", pixel_color);
    });

    eprintln!("\rDone.                        ");
}
