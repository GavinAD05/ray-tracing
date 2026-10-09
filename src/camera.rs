
use rayon::prelude::*;
use std::io;

use crate::color::{self, Color};
use crate::common;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    image_width: i32,
    image_height: i32,
    max_bounces: i32,
    samples_per_pixel: i32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        image_width: i32,
        max_bounces: i32,
        samples_per_pixel: i32,
    ) -> Camera {
        let theta = common::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        let image_height = (image_width as f64 / aspect_ratio) as i32;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            image_width,
            image_height,
            max_bounces,
            samples_per_pixel,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    fn ray_color(r: &Ray, world: &impl Hittable, bounces: i32) -> Color {
        if bounces <= 0 {
            //Out of bounces, no more light gathering
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
            if let Some(scatter_rec) = hit_rec.mat.scatter(r, &hit_rec) {
                return scatter_rec.attenuation * Self::ray_color(&scatter_rec.scattered, world, bounces - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        // Setting background to gradient, lerping from white to blue based on y-component
        let unit_direction = vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(
        &self,
        world: &impl Hittable,
    ) {

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let mut out = io::BufWriter::new(io::stdout().lock());

        for j in (0..self.image_height).rev() {
            eprintln!("Scanlines remaining: {} ", j);
            let pixel_colors: Vec<_> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.samples_per_pixel {
                        let u = (i as f64 + common::random_f64()) / (self.image_width - 1) as f64;
                        let v = (j as f64 + common::random_f64()) / (self.image_height - 1) as f64;
                        let r = self.get_ray(u, v);
                        pixel_color += Self::ray_color(&r, world, self.max_bounces);
                    }
                    pixel_color
                })
                .collect();
            for pixel_color in pixel_colors {
                color::write_color(&mut out, pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("\nDone.");
    }
}
