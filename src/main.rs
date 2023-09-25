use std::iter::Iterator;
use std::time::Instant;

use hit::Hit;
use material::Scatter;
use rand::Rng;
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

fn ray_color(mut r: Ray, world: &[Sphere], depth: u32) -> Color {
    let mut attenuation = Color::WHITE;

    for _ in 0..depth {
        let mut max_t = f32::INFINITY;
        let mut hit = None;

        // Find the closest hitted object.
        for s in world {
            if let Some(s_hit) = s.hit(&r, 0.001..max_t) {
                max_t = s_hit.t;
                hit = Some(s_hit)
            }
        }

        if let Some(hit) = hit {
            let scatter = hit.material.scatter(r, hit.normal, 1.0, hit.front_face);

            match scatter {
                Scatter::Absorbed { solid_color } => {
                    return Color::blend(attenuation, solid_color);
                }
                Scatter::Scattered {
                    direction,
                    attenuation: att,
                } => {
                    attenuation = Color::blend(att, attenuation);
                    r = Ray::new(hit.p, direction);
                    continue;
                }
            };
        }

        let dir = Vec3::unit(r.dir);
        let a = 0.5 * (dir.y() + 1.0);

        let final_color = (1.0 - a) * Color::new((1.0, 1.0, 1.0)) + a * Color::new((0.5, 0.7, 1.0));
        return Color::blend(attenuation, final_color);
    }

    Color::BLACK
}

fn main() {
    let aspect_ratio = 19.0 / 9.0 as f32;
    let width = 800u32;
    let height = (width as f32 / aspect_ratio) as u32;

    let camera = Camera::new(
        aspect_ratio,
        width,
        20.,
        Vec3(13., 2., 3.),
        Vec3(0., 0., 0.),
        0.6,
        10.,
    );

    let ground_material = Material::lambertian(Color::new((0.5, 0.5, 0.5)), None);
    let mut world = vec![
        Sphere::new(Vec3::new((0., -1000., 0.)), 1000., ground_material),
        Sphere::new(Vec3::new((0., 1., 0.)), 1., Material::dielectric(1.5, None)),
        Sphere::new(
            Vec3::new((0.0, 2.3, 0.0)),
            0.3,
            Material::lambertian(Color::new((0.2, 0.2, 0.8)), None),
        ),
        Sphere::new(
            Vec3::new((4., 1., 0.)),
            1.,
            Material::metal(Color::new((0.8, 0.8, 0.8)), None),
        ),
        Sphere::new(
            Vec3::new((-4.0, 1., 0.)),
            1.,
            Material::metal(Color::new((0.8, 0.6, 0.2)), Some(0.3)),
        ),
    ];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            let material = if (0.0..0.7).contains(&choose_mat) {
                Material::lambertian(Color::rand(0.0..1.0), None)
            } else if (0.7..0.9).contains(&choose_mat) {
                Material::metal(Color::rand(0.4..0.8), Some(rng.gen()))
            } else {
                Material::dielectric(rng.gen_range(1.0..5.0), None)
            };

            world.push(Sphere::new(center, 0.2, material));
        }
    }

    print!("P3\n{} {}\n255\n", width, height);

    let samples = 50;
    let depth = 20;
    let time = Instant::now();

    let colors = camera.ray_map(samples, |r| {
        let pixel_color = r
            .iter()
            .map(|r| ray_color(*r, &world, depth))
            .sum::<Color>();

        (1.0 / samples as f32) * pixel_color
    });

    colors.iter().flatten().for_each(|c| println!("{}", c));

    let elapsed = time.elapsed();
    eprintln!("\rDone.                                   ");
    eprintln!(
        "Time took: {}m {:.2}s",
        elapsed.as_secs() / 60,
        elapsed.as_secs_f32() % 60.
    );
}
