use std::iter::Iterator;

use hit::Hit;
use material::Scatter;
use sphere::Sphere;

use crate::camera::Camera;
use crate::material::Material;
use crate::vec3::{Color, Vec3};

use crate::ray::Ray;

mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

fn ray_color(r: Ray, world: &[Sphere], depth: u32) -> Color {
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
        .rev()
        .last();

    if let Some(hit) = hit {
        let scatter = hit.material.scatter(r, hit.normal, 1.0, hit.front_face);

        return match scatter {
            Scatter::Absorbed { solid_color } => solid_color,
            Scatter::Scattered {
                direction,
                attenuation,
            } => Color::blend(
                attenuation,
                ray_color(Ray::new(hit.p, direction), world, depth - 1),
            ),
        };
    }

    let dir = Vec3::unit(r.dir);
    let a = 0.5 * (dir.y() + 1.0);

    (1.0 - a) * Color::new((1.0, 1.0, 1.0)) + a * Color::new((0.5, 0.7, 1.0))
}

fn main() {
    let aspect_ratio = 19.0 / 9.0 as f32;
    let width = 400u32;
    let height = (width as f32 / aspect_ratio) as u32;

    let camera = Camera::new(
        aspect_ratio,
        width,
        20.,
        Vec3(-2., 2., 1.),
        Vec3(0., 0., -1.),
        10.,
        3.4,
    );

    let world = vec![
        Sphere::new(
            Vec3::new((0.0, 0.0, -1.0)),
            0.5,
            Material::metal(Color::new((0.8, 0.8, 0.8)), None),
        ),
        Sphere::new(
            Vec3::new((0.0, 0.7, -1.0)),
            0.3,
            Material::lambertian(Color::new((0.2, 0.2, 0.8)), None),
        ),
        Sphere::new(
            Vec3::new((1.0, 0.0, -1.0)),
            0.5,
            Material::metal(Color::new((0.8, 0.6, 0.2)), Some(0.4)),
        ),
        Sphere::new(
            Vec3::new((-1.0, 0.0, -1.0)),
            0.5,
            Material::dielectric(1.5, None),
        ),
        Sphere::new(
            Vec3::new((-1.0, 0.0, -1.0)),
            -0.4,
            Material::dielectric(1.5, None),
        ),
        Sphere::new(
            Vec3::new((0.0, -100.5, -1.0)),
            100.0,
            Material::lambertian(Color::new((0.9, 0.2, 0.2)), None),
        ),
    ];

    print!("P3\n{} {}\n255\n", width, height);

    let samples = 30;
    let depth = 10;

    let colors = camera.ray_map(samples, |r| {
        let pixel_color = r
            .iter()
            .map(|r| ray_color(*r, &world, depth))
            .sum::<Color>();

        (1.0 / samples as f32) * pixel_color
    });

    colors.iter().flatten().for_each(|c| println!("{}", c));

    eprintln!("\rDone.                                   ");
}
