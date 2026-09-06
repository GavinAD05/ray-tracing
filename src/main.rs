use ray_tracing::{
    camera::Camera,
    render, scene,
    vec3::{Point3, Vec3},
};

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 250;
    const MAX_BOUNCES: i32 = 50;

    // World

    let world = scene::random_scene();

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
    render::render(
        cam,
        world,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        MAX_BOUNCES,
        SAMPLES_PER_PIXEL,
    );
}
