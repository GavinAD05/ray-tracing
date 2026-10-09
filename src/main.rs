use ray_tracing::{
    camera::Camera,
    scene,
    vec3::{Point3, Vec3},
};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 2560;
    const SAMPLES_PER_PIXEL: i32 = 10;
    const MAX_BOUNCES: i32 = 25;

    let world = scene::random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        IMAGE_WIDTH,
        MAX_BOUNCES,
        SAMPLES_PER_PIXEL,
    );

    cam.render(&world);
}
