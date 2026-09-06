use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Material) -> Self {
        Sphere {
            center: cen,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat,
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}

pub enum Shape {
    Sphere(Sphere),
}

impl Hittable for Shape {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shape::Sphere(shape) => shape.hit(r, t_min, t_max),
        }
    }
}

impl From<Sphere> for Shape {
    fn from(value: Sphere) -> Self {
        Shape::Sphere(value)
    }
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Shape>,
}

impl HittableList {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, object: Shape) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
