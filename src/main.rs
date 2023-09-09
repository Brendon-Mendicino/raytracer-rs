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

fn ray_color(r: &Ray, world: &[Sphere]) -> Color {
    let mut max_t = f32::INFINITY;

    let hit = world
        .iter()
        .filter_map(|s| match s.hit(r, 0.0..max_t) {
            Some(hit) => {
                max_t = hit.t;
                Some(hit)
            }
            None => None,
        })
        .min_by(|x, y| x.t.total_cmp(&y.t));

    if let Some(hit) = hit {
        return 0.5 * (Color::new(hit.normal.into()) + Color::new((1.0, 1.0, 1.0)));
    }

    let dir = Vec3::unit(r.dir);
    let a = 0.5 * (dir.y() + 1.0);
    (1.0 - a) * Color::new((1.0, 1.0, 1.0)) + a * Color::new((0.5, 0.7, 1.0))
}

fn main() {
    let aspect_ratio = 19.0 / 9.0 as f32;
    let width = 400u32;
    let height = (width as f32 / aspect_ratio) as u32;

    let camera = Camera::new(aspect_ratio, width);

    let world = vec![
        Sphere::new(Vec3::new((0.0, 0.0, -1.0)), 0.5),
        Sphere::new(Vec3::new((0.0, -100.5, -1.0)), 100.0),
    ];

    print!("P3\n{} {}\n255\n", width, height);

    for r in camera.ray_iter() {
        let pixel_color = ray_color(&r, &world);
        println!("{}", pixel_color);
    }

    eprintln!("\rDone.                        ");
}
