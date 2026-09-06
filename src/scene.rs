use crate::color::Color;
use crate::common;
use crate::hittable::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::shape::{Shape, Sphere};
use crate::vec3::Point3;

pub fn random_scene() -> HittableList<Shape> {
    let mut world = HittableList::new();

    let ground_material = Material::from(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Shape::from(Sphere::new(
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
                    let sphere_material = Material::from(Lambertian::new(albedo));
                    world.add(Shape::from(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = common::random_f64_range(0.0, 0.5);
                    let sphere_material = Material::from(Metal::new(albedo, fuzz));
                    world.add(Shape::from(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Material::from(Dielectric::new(1.5));
                    world.add(Shape::from(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Material::from(Dielectric::new(1.5));
    world.add(Shape::from(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Material::from(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Shape::from(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Material::from(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Shape::from(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
