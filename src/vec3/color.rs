use super::Color;
use crate::usefulstuff::clamp;

pub fn write_color(pixel_color: Color, samples_per_pixel: f64) {
    
    let scale = 1.0 / samples_per_pixel;
    
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();


    let ir = (256.0 * clamp(r, 0.0, 0.999)).round() as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)).round() as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)).round() as i32;
    print!("{} {} {}\n", ir, ig, ib);
}