mod vec3;
mod color;

use crate::vec3::Vec3;
use color::write_color;

fn main() {
    const image_width: u32 = 256;
    const image_height: u32 = 256;
    
    println!("P3\n{image_width} {image_height}\n255");

    for i in 0..image_height{
        eprint!("\rScanlines remaining: {} ", image_height - i);
        for j in 0..image_width{
            let pixel_color: Vec3 = Vec3::new((i as f64)/(image_width-1) as f64,
            (j as f64)/((image_width-1) as f64),0.0);

            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}
