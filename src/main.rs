use std::io;
use std::sync::Arc;

use rayon::prelude::*;

use ray_tracing::{
    camera::Camera,
    color,
    color::Color,
    common,
    hittable::{Hittable, HittableList},
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3,
    vec3::{Point3, Vec3},
};

fn ray_color(r: &Ray, world: &dyn Hittable, bounces: i32) -> Color {
    if bounces <= 0 {
        //Out of bounces, no more light gathering
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(r, 0.001, common::INFINITY) {
        if let Some(scatter_rec) = hit_rec.mat.as_ref().unwrap().scatter(r, &hit_rec) {
            return scatter_rec.attenuation * ray_color(&scatter_rec.scattered, world, bounces - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // Setting background to gradient, lerping from white to blue based on y-component
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = common::random_f64();
            let center = Point3::new(
                a as f64 + 0.9 * common::random_f64(),
                0.2,
                b as f64 + 0.9 * common::random_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_f64_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_BOUNCES: i32 = 50;

    // World

    let world = random_scene();

    // Camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperature,
        dist_to_focus,
    );

    //Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {} ", j);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_f64()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + common::random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_BOUNCES);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}
