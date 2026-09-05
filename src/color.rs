use std::io::Write;

use crate::common;
use crate::vec3::Vec3;

//Will use Vec3 for colors as well as Points, just be careful not to do anything that colors
//shouldn't do.
pub type Color = Vec3;

//Writing [0, 255] value of color component, colors channels must be from [0,1]
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    //Gamma correction to Gamma 2
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    writeln!(
        out,
        "{} {} {}",
        (256.0 * common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("Must be able to write to output");
}
