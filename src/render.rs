use rayon::prelude::*;
use std::io;

use crate::camera::Camera;
use crate::color::{self, Color};
use crate::common;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3;

fn ray_color(r: &Ray, world: &impl Hittable, bounces: i32) -> Color {
    if bounces <= 0 {
        //Out of bounces, no more light gathering
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(r, 0.001, common::INFINITY) {
        if let Some(scatter_rec) = hit_rec.mat.scatter(r, &hit_rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, bounces - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // Setting background to gradient, lerping from white to blue based on y-component
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn render(
    camera: Camera,
    world: impl Hittable,
    image_width: i32,
    aspect_ratio: f64,
    max_bounces: i32,
    samples_per_pixel: i32,
) {
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} ", j);
        let pixel_colors: Vec<_> = (0..image_width)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + common::random_f64()) / (image_width - 1) as f64;
                    let v = (j as f64 + common::random_f64()) / (image_height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, max_bounces);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            color::write_color(&mut io::stdout(), pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
